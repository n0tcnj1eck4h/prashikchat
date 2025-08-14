use super::Guild;
use crate::widgets::GuildButton;
use egui::{Button, ScrollArea, SidePanel, Vec2};

pub struct GuildsPanel<'a> {
    pub guilds: &'a [Guild],
    pub selected_guild: Option<usize>,
}

pub enum GuildsPanelResponse {
    Nothing,
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

    pub fn show(self, ctx: &egui::Context) -> GuildsPanelResponse {
        SidePanel::left("guilds")
            .resizable(false)
            .default_width(64.0)
            .show(ctx, |ui| {
                let mut ret = GuildsPanelResponse::Nothing;
                ui.vertical_centered(|ui| {
                    ScrollArea::vertical().show(ui, |ui| {
                        let size = Vec2::splat(ui.available_width());
                        if ui.add_sized(size, Button::new("")).clicked() {
                            ret = GuildsPanelResponse::Home;
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
                                ret = GuildsPanelResponse::Guild(i);
                            }
                            ui.spacing();
                        }
                        if ui.add_sized(size, Button::new("")).clicked() {
                            ret = GuildsPanelResponse::New;
                        }
                    })
                });
                ret
            })
            .inner
    }
}
