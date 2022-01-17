use std::collections::VecDeque;

//"MCPE;§5raknet rs;390;1.17.42;0;10;13253860892328930865;Bedrock level;Survival;1;19132;19133;"
//"MCPE;§b§lEU §7§l» WINTERFEST \u{e101};121;1.0;20566;100001;1821793688326766702;Hive Games;Survival"
pub struct Motd {
    pub title: String,
    pub protocol_version: u16,
    pub version: String,
    pub online_player: u32,
    pub max_player: u32,
    pub guid: u64,
    pub sub_title: String,
    pub game_mode: String,
}

impl From<&str> for Motd {
    fn from(motd_str: &str) -> Self {
        let mut args: VecDeque<&str> = motd_str.split(';').collect();
        let _ = args.pop_front(); //this will be MCPE
        Self {
            title: args.pop_front().unwrap_or_default().to_owned(),
            protocol_version: args
                .pop_front()
                .unwrap_or_default()
                .parse::<u16>()
                .unwrap_or_default(),
            version: args.pop_front().unwrap_or_default().to_owned(),
            online_player: args
                .pop_front()
                .unwrap_or_default()
                .parse::<u32>()
                .unwrap_or_default(),
            max_player: args
                .pop_front()
                .unwrap_or_default()
                .parse::<u32>()
                .unwrap_or_default(),
            guid: args
                .pop_front()
                .unwrap_or_default()
                .parse::<u64>()
                .unwrap_or_default(),
            sub_title: args.pop_front().unwrap_or_default().to_owned(),
            game_mode: args.pop_front().unwrap_or_default().to_owned(),
        }
    }
}

impl ToString for Motd {
    fn to_string(&self) -> String {
        format!(
            "MCPE;{};{};{};{};{};{};{};{}",
            self.title,
            self.protocol_version,
            self.version,
            self.online_player,
            self.max_player,
            self.guid,
            self.sub_title,
            self.game_mode,
        )
    }
}
