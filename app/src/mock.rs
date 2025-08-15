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
            name: "Naruto".to_string(),
            avatar_url: "https://cdn.donmai.us/sample/04/bf/__uzumaki_naruto_naruto_drawn_by_nanxdaime__sample-04bf37f6579545a0cb592d06d5e0a5a2.jpg".to_string(),
        },
    );

    members.insert(
        2,
        GuildMember {
            name: "Hinata".to_string(),
            avatar_url: "https://cdn.discordapp.com/attachments/1332138826273001472/1405609893250994337/RDMOkNv.jpg?ex=689f73b9&is=689e2239&hm=322e857411f947cb1eaa743cd48358a138f8c009c46db33459f4c270c77a8d98&".to_string(),
        },
    );

    members.insert(
        3,
        GuildMember {
            name: "Susuke".to_string(),
            avatar_url: "https://cdn.donmai.us/original/ef/46/__uchiha_sasuke_naruto_drawn_by_user_tmsf7747__ef463210de5702c88049cdf119b63daf.jpg".to_string(),
        },
    );

    members.insert(
        4,
        GuildMember {
            name: "Chiyo Mihama".to_string(),
            avatar_url: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fi.pinimg.com%2Foriginals%2F89%2Ff4%2Fa5%2F89f4a54f95d04aebba4e9cadd0082e39.jpg&f=1&nofb=1&ipt=d14a02a9ef70d9cb2cd87f2173da2235264c7a13a30258fc105ff1cf80608d84".to_string(),
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
