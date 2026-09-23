use crate::{event::{self, Event}, protocol::{self, Message}};

pub fn parse(message: Message) -> Vec<Event> {
    match message {
        Message::Connected => vec![Event::Connected],
        Message::PlayerList { players } => parse_player_list(players),
        Message::ChatMessageFormatted { message } => parse_chat_message(&message),
    }
}

pub fn parse_chat_message(message: &str) -> Vec<Event> {
    if message.contains("Goodluck with your BedWars Game") {
        return vec![Event::GameStarted];
    }

    vec![]
}

pub fn parse_player_list(players: Vec<protocol::Player>) -> Vec<Event> {
    let players = players.iter().map(|player| {
        event::Player::parse(&player.name, &player.formatted)
    }).collect();

    vec![Event::PlayerList(players)]
}