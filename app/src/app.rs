use super::client::Client;
use crate::Channel;
use crate::ChannelKind;
use crate::Guild;
use crate::GuildMember;
use crate::Message;
use crate::TextChannel;
use crate::client::RecvResult;
use crate::mock::mock_guilds;
use crate::panels::GuildsPanel;
use crate::panels::GuildsPanelResponse;
use crate::widgets::GuildButton;
use crate::widgets::MessageWidget;
use eframe::CreationContext;
use egui::Align;
use egui::Button;
use egui::CentralPanel;
use egui::FontData;
use egui::FontDefinitions;
use egui::Frame;
use egui::Image;
use egui::Key;
use egui::KeyboardShortcut;
use egui::Label;
use egui::Layout;
use egui::Modal;
use egui::Modifiers;
use egui::ScrollArea;
use egui::SidePanel;
use egui::TextBuffer;
use egui::TextEdit;
use egui::TopBottomPanel;
use egui::Vec2;
use egui_extras::install_image_loaders;
use std::sync::Arc;

pub struct App {
    pub buffer: String,
    pub guilds: Vec<Guild>,
    pub selected_guild: Option<usize>,
    pub client: Client,
    pub show_members: bool,
    pub show_new_guild_modal: bool,
    pub me: u32,
}

impl App {
    pub fn new(cc: &CreationContext) -> Self {
        let client =
            Client::new("ws://127.0.0.1:3000/ws").expect("Failed to spawn websocket thread xd");

        install_image_loaders(&cc.egui_ctx);
        install_fonts(cc);

        App {
            buffer: String::new(),
            guilds: mock_guilds(),
            selected_guild: None,
            client,
            show_members: true,
            show_new_guild_modal: false,
            me: 1,
        }
    }
}

fn install_fonts(cc: &CreationContext<'_>) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "font awesome".to_string(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/Font Awesome 7 Free-Solid-900.otf"
        ))),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .push("font awesome".to_string());

    cc.egui_ctx.set_fonts(fonts);
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let select_guild = GuildsPanel::new(&self.guilds, self.selected_guild).show(ctx);

        match select_guild {
            GuildsPanelResponse::Nothing => {}
            GuildsPanelResponse::Home => self.selected_guild = None,
            GuildsPanelResponse::Guild(i) => self.selected_guild = Some(i),
            GuildsPanelResponse::New => {
                self.show_new_guild_modal = true;
            }
        }

        if let Some(guild_id) = self.selected_guild {
            let guild = &mut self.guilds[guild_id];

            let select_channel = channels(ctx, guild).inner;

            if self.show_members {
                MembersPanel::new(guild.members.values()).show(ctx);
            }

            let msg = message_box(ctx, &mut self.buffer);

            if let Some(res) = central_panel(ctx, guild) {
                match res {
                    CentralPanelResponse::ToggleMemberList => {
                        self.show_members ^= true;
                    }
                }
            }

            if let Some(ch) = select_channel {
                guild.focused_channel_idx = ch;
            }

            match msg {
                Some(MessageBoxResponse::Send(msg)) => {
                    // self.client.send.send(WsMessage::Text(msg));
                    if let ChannelKind::Text(channel) =
                        &mut guild.channels[guild.focused_channel_idx].kind
                    {
                        channel.messages.push(Message {
                            author_id: self.me,
                            content: msg,
                        });
                    }
                }
                Some(MessageBoxResponse::Emoji) => {
                    self.me %= guild.members.len() as u32;
                    self.me += 1;
                }
                Some(MessageBoxResponse::PickFile) => {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.buffer = path.to_string_lossy().to_string();
                    }
                }
                None => {}
            }
        } else {
            CentralPanel::default().show(ctx, |ui| {
                ui.add_sized(ui.available_size(), Label::new("Home"));
            });
        }

        if self.show_new_guild_modal {
            let response = Modal::new("new guild modal".into()).show(ctx, |ui| {
                ui.heading("haii");
            });
            if response.backdrop_response.clicked() {
                self.show_new_guild_modal = false;
            }
        }

        loop {
            match self.client.recieve() {
                RecvResult::OkNone => {
                    break;
                }
                RecvResult::Connecting => {
                    break;
                }
                RecvResult::Disconnected => {
                    break;
                }
                RecvResult::Error(_) => {
                    break;
                }
                RecvResult::Ok(server_message) => {
                    dbg!(server_message);
                }
            }
        }
    }
}

pub enum CentralPanelResponse {
    ToggleMemberList,
}

fn central_panel(ctx: &egui::Context, guild: &Guild) -> Option<CentralPanelResponse> {
    let channel = &guild.channels[guild.focused_channel_idx];
    let members = &guild.members;
    let mut ret = None;
    CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading(&channel.name);
            ui.separator();
            ui.weak(&channel.description);
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui
                    .add_sized(Vec2::splat(ui.available_height()), Button::new(""))
                    .clicked()
                {
                    ret = Some(CentralPanelResponse::ToggleMemberList);
                };
                ui.add_sized(Vec2::splat(ui.available_height()), Button::new(""));
            });
        });
        ui.separator();
        match channel.kind {
            ChannelKind::Text(ref channel) => {
                ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                    for msg in &channel.messages {
                        let author = members.get(&msg.author_id).unwrap();
                        ui.add(MessageWidget::new(&msg, &author));
                        ui.spacing();
                    }
                });
            }
            ChannelKind::Voice => {
                ui.add_sized(ui.available_size(), Label::new("Voice"));
            }
        }
    });

    return ret;
}

pub struct MembersPanel<T>(T);

impl<'a, I: IntoIterator<Item = &'a GuildMember>> MembersPanel<I> {
    fn new(members: I) -> Self {
        Self(members)
    }

    fn show(self, ctx: &egui::Context) {
        SidePanel::right("members")
            .resizable(false)
            .default_width(128.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ScrollArea::vertical().show(ui, |ui| {
                        for member in self.0 {
                            ui.horizontal(|ui| {
                                ui.image(&member.avatar_url);
                                ui.label(&member.name);
                            });
                        }
                    })
                });
            });
    }
}

pub enum MessageBoxResponse {
    Send(String),
    PickFile,
    Emoji,
}

pub fn message_box(ctx: &egui::Context, buffer: &mut String) -> Option<MessageBoxResponse> {
    let mut frame = Frame::side_top_panel(&ctx.style());
    frame.inner_margin.left = 0;
    TopBottomPanel::bottom("textbox")
        .exact_height(64.0)
        .frame(frame)
        .show(ctx, |ui| {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let mut ret = None;
                let h = ui.available_height();
                if ui.add_sized(Vec2::splat(h), Button::new("")).clicked() {
                    ret = Some(MessageBoxResponse::PickFile);
                }

                if ui.add_sized(Vec2::splat(h), Button::new("")).clicked() {
                    ret = Some(MessageBoxResponse::Emoji);
                }

                let send = ui.add_sized(Vec2::splat(h), Button::new(""));
                ui.add_sized(
                    ui.available_size(),
                    TextEdit::multiline(buffer)
                        .return_key(Some(KeyboardShortcut::new(Modifiers::CTRL, Key::Enter))),
                );

                if ui.input(|i| i.key_pressed(Key::Enter) && !i.modifiers.ctrl) || send.clicked() {
                    ret = Some(MessageBoxResponse::Send(buffer.take()));
                }

                return ret;
            })
        })
        .inner
        .inner
}

fn channels(ctx: &egui::Context, guild: &Guild) -> egui::InnerResponse<Option<usize>> {
    SidePanel::left("channels")
        .resizable(false)
        .default_width(128.0 + 32.0)
        .show(ctx, |ui| {
            let mut s = None;
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.heading(&guild.name);
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.button("");
                    });
                });
                ui.separator();
                ScrollArea::vertical().show(ui, |ui| {
                    for (i, channel) in guild.channels.iter().enumerate() {
                        let icon = match channel.kind {
                            ChannelKind::Text(_) => "",
                            ChannelKind::Voice => "",
                        };
                        if ui
                            .selectable_label(
                                i == guild.focused_channel_idx,
                                format!("{} {}", icon, channel.name),
                            )
                            .clicked()
                        {
                            s = Some(i);
                        }
                    }
                })
            });
            s
        })
}
