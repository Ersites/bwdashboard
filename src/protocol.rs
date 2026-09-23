use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "player_list")]
    PlayerList { players: Vec<Player> },
    #[serde(rename = "chat_message_formatted")]
    ChatMessageFormatted { message: String },
}

#[derive(Debug, Deserialize)]
pub struct Player {
    pub name: String,
    pub formatted: String,
}
