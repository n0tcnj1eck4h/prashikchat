use super::Guild;
use crate::{
    ChannelKind, GuildMember,
    widgets::{GuildButton, MessageWidget},
};
use egui::{
    Align, Button, CentralPanel, Frame, Key, KeyboardShortcut, Label, Layout, Modifiers,
    ScrollArea, SidePanel, TextBuffer, TextEdit, TopBottomPanel, Vec2,
};

pub struct GuildsPanel<'a> {
    pub guilds: &'a [Guild],
    pub selected_guild: Option<usize>,
}

pub enum GuildsPanelResponse {
    Home,
    Guild(usize),
    New,
}

impl<'a> GuildsPanel<'a> {
    pub fn new(guilds: &'a [Guild], selected_guild: Option<usize>) -> Self {
        Self {
            guilds,
            selected_guild,
        }
    }

    pub fn show(self, ctx: &egui::Context) -> Option<GuildsPanelResponse> {
        let mut ret = None;
        SidePanel::left("guilds")
            .resizable(false)
            .default_width(64.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ScrollArea::vertical().show(ui, |ui| {
                        let size = Vec2::splat(ui.available_width());
                        if ui.add_sized(size, Button::new("")).clicked() {
                            ret = Some(GuildsPanelResponse::Home);
                        }
                        ui.separator();
                        for (i, guild) in self.guilds.iter().enumerate() {
                            if ui
                                .add(
                                    GuildButton::from_url(&guild.icon_url)
                                        .selected(self.selected_guild == Some(i)),
                                )
                                .clicked()
                            {
                                ret = Some(GuildsPanelResponse::Guild(i));
                            }
                            ui.spacing();
                        }
                        if ui.add_sized(size, Button::new("")).clicked() {
                            ret = Some(GuildsPanelResponse::New);
                        }
                    })
                });
            });
        ret
    }
}

pub enum AwesomePanelResponse {
    ToggleMemberList,
}

pub struct AwesomeCentralPanel<'a> {
    guild: &'a Guild,
}

impl<'a> AwesomeCentralPanel<'a> {
    pub fn new(guild: &'a Guild) -> Self {
        Self { guild }
    }

    pub fn show(self, ctx: &egui::Context) -> Option<AwesomePanelResponse> {
        let channel = &self.guild.channels[self.guild.focused_channel_idx];
        let members = &self.guild.members;
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
                        ret = Some(AwesomePanelResponse::ToggleMemberList);
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
                            ui.add(MessageWidget::new(msg, author));
                            ui.spacing();
                        }
                    });
                }
                ChannelKind::Voice => {
                    ui.add_sized(ui.available_size(), Label::new("Voice"));
                }
            }
        });

        ret
    }
}

pub struct MembersPanel<T>(T);

impl<'a, I: IntoIterator<Item = &'a GuildMember>> MembersPanel<I> {
    pub fn new(members: I) -> Self {
        Self(members)
    }

    pub fn show(self, ctx: &egui::Context) {
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

pub struct MessageBox<'a> {
    buffer: &'a mut String,
}

impl<'a> MessageBox<'a> {
    pub fn new(buffer: &'a mut String) -> Self {
        Self { buffer }
    }

    pub fn show(self, ctx: &egui::Context) -> Option<MessageBoxResponse> {
        let mut ret = None;
        let mut frame = Frame::side_top_panel(&ctx.style());
        frame.inner_margin.left = 0;
        TopBottomPanel::bottom("textbox")
            .exact_height(64.0)
            .frame(frame)
            .show(ctx, |ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let size = Vec2::splat(ui.available_height());
                    if ui.add_sized(size, Button::new("")).clicked() {
                        ret = Some(MessageBoxResponse::PickFile);
                    }

                    if ui.add_sized(size, Button::new("")).clicked() {
                        ret = Some(MessageBoxResponse::Emoji);
                    }

                    let send = ui.add_sized(size, Button::new(""));
                    ui.add_sized(
                        ui.available_size(),
                        TextEdit::multiline(self.buffer)
                            .return_key(Some(KeyboardShortcut::new(Modifiers::CTRL, Key::Enter))),
                    );

                    if ui.input(|i| i.key_pressed(Key::Enter) && !i.modifiers.ctrl)
                        || send.clicked()
                    {
                        ret = Some(MessageBoxResponse::Send(self.buffer.take()));
                    }
                });
            });
        ret
    }
}

pub struct ChannelsPanel<'a> {
    guild: &'a Guild,
}

impl<'a> ChannelsPanel<'a> {
    pub fn new(guild: &'a Guild) -> Self {
        Self { guild }
    }

    pub fn show(self, ctx: &egui::Context) -> Option<usize> {
        let mut ret = None;
        SidePanel::left("channels")
            .resizable(false)
            .default_width(128.0 + 32.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.heading(&self.guild.name);
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.button("");
                        });
                    });
                    ui.separator();
                    ScrollArea::vertical().show(ui, |ui| {
                        for (i, channel) in self.guild.channels.iter().enumerate() {
                            let icon = match channel.kind {
                                ChannelKind::Text(_) => "",
                                ChannelKind::Voice => "",
                            };
                            if ui
                                .selectable_label(
                                    i == self.guild.focused_channel_idx,
                                    format!("{} {}", icon, channel.name),
                                )
                                .clicked()
                            {
                                ret = Some(i);
                            }
                        }
                    })
                });
            });
        ret
    }
}
