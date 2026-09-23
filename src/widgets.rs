use eframe::egui;

use crate::{
    model::{Model, Player, Team},
    theme::{self, TeamPalette},
};

pub fn header(ui: &mut egui::Ui, model: &Model) {
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
            ui.vertical(|ui| {
                connection_status(ui, model.connected);
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Game")
                            .font(egui::FontId::new(
                                30.0,
                                egui::FontFamily::Name("bold".into()),
                            ))
                            .color(theme::PRIMARY),
                    );
                    ui.label(
                        egui::RichText::new(format!("{}", model.game_number))
                            .font(egui::FontId::new(
                                30.0,
                                egui::FontFamily::Name("bold".into()),
                            ))
                            .color(theme::PRIMARY),
                    );
                });

                ui.add_space(10.0);
            });
        });
        ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
            // timers(ui);
        });
    });
}

pub fn timers(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("25:57")
                .font(egui::FontId::new(
                    20.0,
                    egui::FontFamily::Name("bold".into()),
                ))
                .color(theme::PRIMARY),
        );

        ui.label(egui::RichText::new("END: ").color(theme::SECONDARY));
    });
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("14:27")
                .font(egui::FontId::new(
                    20.0,
                    egui::FontFamily::Name("bold".into()),
                ))
                .color(theme::PRIMARY),
        );

        ui.label(egui::RichText::new("BED: ").color(theme::SECONDARY));
    });
}

pub fn connection_status(ui: &mut egui::Ui, connected: bool) {
    ui.horizontal(|ui| {
        let (rect, _) =
            ui.allocate_exact_size(egui::Vec2 { x: 16.0, y: 16.0 }, egui::Sense::hover());

        let painter = ui.painter();

        if connected {
            painter.circle(
                rect.center(),
                6.0,
                theme::CONNECTED,
                egui::Stroke::default(),
            );
            ui.label(egui::RichText::new("Connected").color(theme::CONNECTED));
        } else {
            painter.circle(
                rect.center(),
                6.0,
                theme::DISCONNECTED,
                egui::Stroke::default(),
            );
            ui.label(egui::RichText::new("Disconnected").color(theme::DISCONNECTED));
        };
    });
}

pub fn team(ui: &mut egui::Ui, team: &Team) {
    let palette = theme::get_palette(team.color);

    egui::Frame::new()
        .fill(palette.background)
        .stroke(egui::Stroke::new(1.0, palette.border))
        .corner_radius(5)
        .inner_margin(10)
        .show(ui, |ui| {
            team_name(ui, palette.name.to_owned(), palette);
            ui.add_space(5.0);
            win_probability(ui, team.skill, None, palette);
            ui.add_space(10.0);

            for p in &team.players {
                player(ui, p, palette);
            }
        });
}

pub fn team_name(ui: &mut egui::Ui, name: String, palette: TeamPalette) {
    ui.horizontal(|ui| {
        let (rect, _) =
            ui.allocate_exact_size(egui::Vec2 { x: 16.0, y: 16.0 }, egui::Sense::hover());

        let painter = ui.painter();
        painter.circle(rect.center(), 6.0, palette.primary, egui::Stroke::default());

        ui.label(
            egui::RichText::new(name)
                .color(palette.primary)
                .font(egui::FontId::new(
                    16.0,
                    egui::FontFamily::Name("bold".into()),
                )),
        );
    });
}

pub fn player(ui: &mut egui::Ui, player: &Player, palette: TeamPalette) {
    egui::Frame::new()
        .fill(palette.card_background)
        .stroke(egui::Stroke::new(1.0, palette.card_border))
        .corner_radius(5)
        .inner_margin(egui::Margin::symmetric(10, 5))
        .outer_margin(1)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    player_name(ui, &player.name, palette);
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    skill(ui, player.skill, palette);
                })
            })
        });
}

pub fn player_name(ui: &mut egui::Ui, name: &str, palette: TeamPalette) {
    ui.label(egui::RichText::new(name).color(palette.text).size(18.0));
}

pub fn win_probability(
    ui: &mut egui::Ui,
    s: Option<u32>,
    probability: Option<u32>,
    palette: TeamPalette,
) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                skill(ui, s, palette);
            });
            let probability = match probability {
                Some(value) => &format!("{}%", value),
                None => "- %",
            };
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(probability)
                        .color(palette.primary)
                        .font(egui::FontId::new(
                            20.0,
                            egui::FontFamily::Name("bold".into()),
                        )),
                );
            });
        });

        ui.visuals_mut().extreme_bg_color = palette.card_background;
        ui.add(
            egui::ProgressBar::new(0.67)
                .fill(palette.primary)
                .desired_height(8.0),
        );
    });
}

pub fn skill(ui: &mut egui::Ui, skill: Option<u32>, palette: TeamPalette) {
    let skill = match skill {
        Some(value) => &value.to_string(),
        None => "-",
    };

    ui.label(
        egui::RichText::new(skill)
            .color(palette.primary)
            .font(egui::FontId::new(
                20.0,
                egui::FontFamily::Name("bold".into()),
            )),
    );
}
