use egui::Align;
use egui::ImageButton;

use egui::Layout;
use egui::Vec2;

use egui::Widget;

use egui::Image;

use crate::GuildMember;
use crate::Message;

pub struct GuildButton<'a>(Image<'a>, bool);

impl<'a> GuildButton<'a> {
    pub fn new(image: impl Into<Image<'a>>) -> GuildButton<'a> {
        GuildButton(image.into(), false)
    }

    pub fn from_url(image: &'a str) -> GuildButton<'a> {
        Self::new(Image::new(image))
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.1 = selected;
        self
    }
}

impl Widget for GuildButton<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let size = Vec2::splat(ui.available_width());
        ui.add_sized(size, ImageButton::new(self.0).selected(self.1))
    }
}

pub struct MessageWidget<'a> {
    msg: &'a Message,
    author: &'a GuildMember,
}

impl<'a> MessageWidget<'a> {
    pub fn new(msg: &'a Message, author: &'a GuildMember) -> Self {
        Self { msg, author }
    }
}

impl<'a> Widget for MessageWidget<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            ui.add_sized(Vec2::splat(32.0), Image::new(&self.author.avatar_url));
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.heading(&self.author.name);
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.spacing();
                        ui.button("");
                        ui.weak("23:22");
                    });
                });
                ui.label(&self.msg.content);
            });
        })
        .response
    }
}
