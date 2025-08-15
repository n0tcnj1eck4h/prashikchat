use super::client::Client;
use crate::ChannelKind;
use crate::Guild;
use crate::Message;
use crate::client::RecvResult;
use crate::mock::mock_guilds;
use crate::panels::AwesomeCentralPanel;
use crate::panels::AwesomePanelResponse;
use crate::panels::ChannelsPanel;
use crate::panels::GuildsPanel;
use crate::panels::GuildsPanelResponse;
use crate::panels::MembersPanel;
use crate::panels::MessageBox;
use crate::panels::MessageBoxResponse;
use eframe::CreationContext;
use egui::CentralPanel;
use egui::FontData;
use egui::FontDefinitions;
use egui::Label;
use egui::Modal;
use egui::ModalResponse;
use egui_extras::install_image_loaders;
use std::sync::Arc;

#[derive(Clone, Copy)]
pub enum CurrentModal {
    CreateOrJoinModal,
    CreateModal,
    JoinModal,
}

pub struct App {
    pub buffer: String,
    pub guild_image_buffer: String,
    pub guilds: Vec<Guild>,
    pub selected_guild: Option<usize>,
    pub client: Client,
    pub show_members: bool,
    pub show_current_modal: Option<CurrentModal>,
    pub me: u32,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        self.panels(ctx);
        self.modals(ctx);
        self.update_client();
    }
}

impl App {
    pub fn new(cc: &CreationContext) -> Self {
        let client =
            Client::new("ws://127.0.0.1:3000/ws").expect("Failed to spawn websocket thread xd");

        install_image_loaders(&cc.egui_ctx);
        install_fonts(cc);

        App {
            buffer: String::new(),
            guild_image_buffer: String::new(),
            guilds: mock_guilds(),
            selected_guild: None,
            client,
            show_members: true,
            show_current_modal: None,
            me: 1,
        }
    }

    fn update_client(&mut self) {
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

    fn modals(&mut self, ctx: &egui::Context) {
        if let Some(current_modal) = self.show_current_modal {
            let backdrop_response = match current_modal {
                CurrentModal::CreateOrJoinModal => {
                    let response = create_or_join_guild_modal(ctx);
                    if let Some(res) = response.inner {
                        match res {
                            CreateOrJoin::Create => {
                                self.show_current_modal = Some(CurrentModal::CreateModal)
                            }
                            CreateOrJoin::Join => {
                                self.show_current_modal = Some(CurrentModal::JoinModal)
                            }
                        }
                    }
                    response.backdrop_response
                }
                CurrentModal::CreateModal => {
                    create_guild_modal(ctx, &mut self.buffer, &mut self.guild_image_buffer)
                        .backdrop_response
                }
                CurrentModal::JoinModal => {
                    unreachable!()
                }
            };

            if backdrop_response.clicked() {
                self.show_current_modal = None;
            }
        }
    }

    fn panels(&mut self, ctx: &egui::Context) {
        if let Some(select_guild) = GuildsPanel::new(&self.guilds, self.selected_guild).show(ctx) {
            match select_guild {
                GuildsPanelResponse::Home => self.selected_guild = None,
                GuildsPanelResponse::Guild(i) => self.selected_guild = Some(i),
                GuildsPanelResponse::New => {
                    self.show_current_modal = Some(CurrentModal::CreateOrJoinModal);
                }
            }
        }

        if let Some(guild_id) = self.selected_guild {
            let guild = &mut self.guilds[guild_id];

            if let Some(ch) = ChannelsPanel::new(guild).show(ctx) {
                guild.focused_channel_idx = ch;
            }

            if self.show_members {
                MembersPanel::new(guild.members.values()).show(ctx);
            }

            if let Some(msg) = MessageBox::new(&mut self.buffer).show(ctx) {
                match msg {
                    MessageBoxResponse::Send(msg) => {
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
                    MessageBoxResponse::Emoji => {
                        self.me %= guild.members.len() as u32;
                        self.me += 1;
                    }
                    MessageBoxResponse::PickFile => {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.buffer = path.to_string_lossy().to_string();
                        }
                    }
                }
            }

            if let Some(res) = AwesomeCentralPanel::new(guild).show(ctx) {
                match res {
                    AwesomePanelResponse::ToggleMemberList => {
                        self.show_members ^= true;
                    }
                }
            }
        } else {
            CentralPanel::default().show(ctx, |ui| {
                ui.add_sized(ui.available_size(), Label::new("Home"));
            });
        }
    }
}

fn create_guild_modal(
    ctx: &egui::Context,
    text: &mut String,
    guild_image_buffer: &mut String,
) -> egui::ModalResponse<()> {
    Modal::new("create guild modal".into()).show(ctx, |ui| {
        ui.vertical(|ui| {
            ui.heading("Create a server");
            ui.separator();
            ui.vertical(|ui| {
                ui.label("Server picture");
                ui.horizontal(|ui| {
                    if ui.button("Pick image").clicked() {
                        let pic_path = rfd::FileDialog::new().pick_file();
                        if let Some(horse) = pic_path {
                            *guild_image_buffer = horse.to_string_lossy().to_string();
                        }
                    }
                    ui.label(guild_image_buffer.as_str());
                });

                ui.label("Server name");
                ui.text_edit_singleline(text); // CHANGE BUFFER LATAR
                ui.label("Server description");
                ui.text_edit_singleline(text); // CHANGE BUFFER LATAR
            })
        });
    })
}

pub enum CreateOrJoin {
    Create,
    Join,
}

fn create_or_join_guild_modal(ctx: &egui::Context) -> ModalResponse<Option<CreateOrJoin>> {
    Modal::new("create or join guild modal".into()).show(ctx, |ui| {
        let mut ret = None;
        ui.vertical(|ui| {
            ui.heading("Create or join a server");
            ui.separator();
            ui.vertical_centered_justified(|ui| {
                if ui.button("Create").clicked() {
                    ret = Some(CreateOrJoin::Create);
                };
                if ui.button("Join").clicked() {
                    ret = Some(CreateOrJoin::Join);
                };
            });
        });
        ret
    })
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
