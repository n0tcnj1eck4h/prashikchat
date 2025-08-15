mod app;
mod client;
mod mock;
mod panels;
mod widgets;

pub struct Message {
    pub author_id: u32,
    pub content: String,
}

pub struct TextChannel {
    pub messages: Vec<Message>,
}

pub enum ChannelKind {
    Text(TextChannel),
    Voice,
}

pub struct Channel {
    pub name: String,
    pub kind: ChannelKind,
    pub description: String,
}

pub struct GuildMember {
    pub name: String,
    pub avatar_url: String,
}

pub struct Guild {
    pub id: u32,
    pub name: String,
    pub icon_url: String,
    pub channels: Vec<Channel>,
    pub members: HashMap<u32, GuildMember>,
    pub focused_channel_idx: usize,
}

use std::collections::HashMap;

use eframe::NativeOptions;

fn main() {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "not poopchat",
        native_options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
    .unwrap();
}
