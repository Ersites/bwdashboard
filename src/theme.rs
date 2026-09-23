use eframe::egui::Color32;

use crate::model::TeamColor;

pub const PRIMARY: Color32 = Color32::from_rgb(255, 255, 255);
pub const SECONDARY: Color32 = Color32::from_rgb(147, 147, 147);
pub const BACKGROUND: Color32 = Color32::from_rgb(10, 10, 10);

pub const CONNECTED: Color32 = Color32::from_rgb(34, 197, 94);
pub const DISCONNECTED: Color32 = Color32::from_rgb(239, 68, 68);

#[derive(Debug, Default, Clone, Copy)]
pub struct TeamPalette {
    pub text: Color32,
    pub primary: Color32,
    pub background: Color32,
    pub border: Color32,
    pub card_background: Color32,
    pub card_border: Color32,
    pub name: &'static str,
}

pub const TEAM_RED: TeamPalette = TeamPalette {
    text: Color32::from_rgb(255, 255, 255),
    primary: Color32::from_rgb(239, 68, 68),
    background: Color32::from_rgb(26, 10, 10),
    border: Color32::from_rgb(79, 26, 27),
    card_background: Color32::from_rgb(33, 18, 18),
    card_border: Color32::from_rgb(44, 30, 30),
    name: "RED",
};

pub const TEAM_ORANGE: TeamPalette = TeamPalette {
    text: Color32::from_rgb(255, 255, 255),
    primary: Color32::from_rgb(234, 108, 8),
    background: Color32::from_rgb(20, 13, 10),
    border: Color32::from_rgb(74, 38, 9),
    card_background: Color32::from_rgb(27, 21, 18),
    card_border: Color32::from_rgb(39, 36, 30),
    name: "ORANGE",
};

pub const TEAM_YELLOW: TeamPalette = TeamPalette {
    text: Color32::from_rgb(255, 255, 255),
    primary: Color32::from_rgb(234, 179, 8),
    background: Color32::from_rgb(20, 17, 10),
    border: Color32::from_rgb(74, 58, 9),
    card_background: Color32::from_rgb(27, 24, 18),
    card_border: Color32::from_rgb(39, 36, 30),
    name: "YELLOW",
};

pub const TEAM_GREEN: TeamPalette = TeamPalette {
    text: Color32::from_rgb(255, 255, 255),
    primary: Color32::from_rgb(34, 197, 94),
    background: Color32::from_rgb(10, 20, 16),
    border: Color32::from_rgb(16, 64, 36),
    card_background: Color32::from_rgb(18, 27, 23),
    card_border: Color32::from_rgb(30, 39, 35),
    name: "GREEN",
};

pub const TEAM_BLUE: TeamPalette = TeamPalette {
    text: Color32::from_rgb(255, 255, 255),
    primary: Color32::from_rgb(59, 130, 246),
    background: Color32::from_rgb(9, 15, 26),
    border: Color32::from_rgb(22, 44, 81),
    card_background: Color32::from_rgb(17, 23, 33),
    card_border: Color32::from_rgb(29, 35, 44),
    name: "BLUE",
};

pub const TEAM_INACTIVE: TeamPalette = TeamPalette {
    text: Color32::from_rgb(128, 128, 128),
    primary: Color32::from_rgb(62, 62, 62),
    background: Color32::from_rgb(8, 8, 8),
    border: Color32::from_rgb(20, 20, 20),
    card_background: Color32::from_rgb(11, 11, 11),
    card_border: Color32::from_rgb(16, 16, 16),
    name: "INACTIVE",
};

pub fn get_palette(team_color: TeamColor) -> TeamPalette {
    match team_color {
        TeamColor::Red => TEAM_RED,
        TeamColor::Orange => TEAM_ORANGE,
        TeamColor::Yellow => TEAM_YELLOW,
        TeamColor::Green => TEAM_GREEN,
        TeamColor::Blue => TEAM_BLUE,
        TeamColor::None => TEAM_INACTIVE,
    }
}