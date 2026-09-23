#[derive(Debug)]
pub enum Event {
    Connected,
    Disconnected,
    GameStarted,
    PlayerList(Vec<Player>),
    PlayerSkill{
        name: String,
        skill: u32,
    }
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub team: TeamColor,
}

impl Player {
    pub fn parse(name: &str, formatted: &str) -> Self {
        let team = [
            ("§c§lR§c", TeamColor::Red),
            ("§6§lO§6", TeamColor::Orange),
            ("§e§lY§e", TeamColor::Yellow),
            ("§a§lG§a", TeamColor::Green),
            ("§9§lB§9", TeamColor::Blue),
        ]
        .into_iter()
        .find_map(|(prefix, team)| (formatted == format!("{prefix} {name}")).then_some(team))
        .unwrap_or(TeamColor::None);

        Self {
            name: name.to_owned(),
            team,
        }
    }
}

#[derive(Debug)]
pub enum TeamColor {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    None,
}

impl Event {
    pub fn set_player_skill(name: String, skill: u32) -> Self {
        Event::PlayerSkill { name, skill }
    }
}