use crate::Channel;
use crate::ChannelKind;
use crate::Message;
use crate::TextChannel;

use super::GuildMember;

use std::collections::HashMap;

use super::Guild;

pub fn mock_guilds() -> Vec<Guild> {
    let mut members = HashMap::new();
    members.insert(
        1,
        GuildMember {
            name: "Fresko".to_string(),
            avatar_url: "https://cdn.donmai.us/original/25/f1/__yua_serufu_and_ted_kaczynski_real_life_and_1_more_drawn_by_drawfag__25f156d8dc5df521a04e0000719eb6f0.png".to_string(),
        },
    );

    members.insert(
        2,
        GuildMember {
            name: "Awesome person".to_string(),
            avatar_url: "https://a.ppy.sh/7562902".to_string(),
        },
    );

    let guilds = vec![Guild {
        id: 1,
        name: "Fresko servr".to_string(),
        icon_url: "https://cdn.donmai.us/original/fc/4b/__izuna_blue_archive_drawn_by_aven_r18g__fc4b072b0db2543d374cd3c75b997745.jpg".to_string(),
        channels: vec![
            Channel {
                name: "general".to_string(),
                kind: ChannelKind::Text(TextChannel {
                    messages: vec![
                        Message {
                            author_id: 1,
                            content: "yo waddup bro".to_string(),
                        },
                        Message {
                            author_id: 2,
                            content: "uhhgh im soo bloated and full".to_string(),
                        },
                        Message {
                            author_id: 1,
                            content: "I need to rub my belly".to_string(),
                        },
                    ],
                }),
                description: "very awesome text chanel".to_string(),
            },
            Channel {
                name: "vois".to_string(),
                kind: ChannelKind::Voice,
                description: "blabllg".to_string(),
            },
        ],
        members,
        focused_channel_idx: 0,
    }];
    guilds
}
