use crate::event;

#[derive(Debug, Default, Clone)]
pub struct Model {
    pub connected: bool,
    pub game_number: u32,
    pub teams: Vec<Team>,
}

impl Model {
    pub fn new() -> Self {
        let mut teams = Vec::new();
        for _ in 0..4 {
            teams.push(Team::default());
        }

        Self {
            connected: false,
            game_number: 0,
            teams,
        }
    }

    pub fn set_players(&mut self, players: Vec<event::Player>) {
        self.teams.clear();

        let mut red = Team::default();
        let mut yellow_orange = Team::default();
        let mut green = Team::default();
        let mut blue = Team::default();

        for player in players {
            match player.team {
                event::TeamColor::Red => {
                    red.players.push(Player::new(player.name));
                    red.color = TeamColor::Red;
                }
                event::TeamColor::Orange => {
                    yellow_orange.players.push(Player::new(player.name));
                    yellow_orange.color = TeamColor::Orange;
                }
                event::TeamColor::Yellow => {
                    yellow_orange.players.push(Player::new(player.name));
                    yellow_orange.color = TeamColor::Yellow;
                }
                event::TeamColor::Green => {
                    green.players.push(Player::new(player.name));
                    green.color = TeamColor::Green;
                }
                event::TeamColor::Blue => {
                    blue.players.push(Player::new(player.name));
                    blue.color = TeamColor::Blue;
                }
                _ => {}
            }
        }

        self.teams.push(red);
        self.teams.push(yellow_orange);
        self.teams.push(green);
        self.teams.push(blue);
    }

    pub fn set_player_skill(&mut self, name: &str, skill: u32) {
        for team in &mut self.teams {
            if let Some(player) = team.players.iter_mut().find(|p| p.name == name) {
                player.skill = Some(skill);

                let mut total_skill = 0;
                let mut total_players = 0;

                for p in &team.players {
                    if let Some(skill) = p.skill {
                        total_skill += skill;
                        total_players += 1;
                    }
                }

                team.skill = total_skill.checked_div(total_players)
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Team {
    pub color: TeamColor,
    pub players: Vec<Player>,
    pub skill: Option<u32>,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum TeamColor {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    #[default]
    None,
}

#[derive(Debug, Default, Clone)]
pub struct Player {
    pub name: String,
    pub skill: Option<u32>,
}

impl Player {
    fn new(name: String) -> Self {
        Player { name, skill: None }
    }
}
