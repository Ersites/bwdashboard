use eframe::egui;
use tokio::sync::mpsc;

use crate::{
    event::Event,
    model::Model,
    runtime::{self, ClientEvent},
    theme, widgets,
};

pub struct MyApp {
    model: Model,
    client_rx: crossbeam::channel::Receiver<Event>,
    client_tx: mpsc::Sender<ClientEvent>,
}

impl MyApp {
    pub fn new(ctx: egui::Context) -> Self {
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "cascadia_code".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/fonts/CascadiaCode.ttf")).into(),
        );

        fonts.font_data.insert(
            "cascadia_code_bold".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/fonts/CascadiaCode-Bold.ttf"))
                .into(),
        );

        fonts.families.insert(
            egui::FontFamily::Name("bold".into()),
            vec!["cascadia_code_bold".to_owned()],
        );

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "cascadia_code".to_owned());

        ctx.set_fonts(fonts);

        let (ready_tx, ready_rx) = crossbeam::channel::bounded(1);
        let (tx, client_rx) = crossbeam::channel::unbounded();

        runtime::start(ready_tx, tx, ctx.clone());
        let client_tx = ready_rx.recv().unwrap();

        MyApp {
            model: Model::new(),
            client_rx,
            client_tx,
        }
    }

    fn handle_events(&mut self) {
        while let Ok(event) = self.client_rx.try_recv() {
            match event {
                Event::Connected => {
                    self.model.connected = true;
                }
                Event::Disconnected => {
                    self.model.connected = false;
                }
                Event::GameStarted => {
                    self.model.game_number += 1;
                }
                Event::PlayerList(players) => {
                    self.model.set_players(players);
                    for team in &self.model.teams {
                        for player in &team.players {
                            self.client_tx
                                .blocking_send(ClientEvent::get_player_skill(&player.name))
                                .unwrap();
                        }
                    }
                }
                Event::PlayerSkill { name, skill } => {
                    self.model.set_player_skill(&name, skill);
                }
            }
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        self.handle_events();

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(theme::BACKGROUND).inner_margin(10))
            .show(ui, |ui| {
                widgets::header(ui, &self.model);
                ui.horizontal(|ui| {
                    ui.columns(4, |columns| {
                        widgets::team(&mut columns[0], &self.model.teams[0]);
                        widgets::team(&mut columns[1], &self.model.teams[1]);
                        widgets::team(&mut columns[2], &self.model.teams[2]);
                        widgets::team(&mut columns[3], &self.model.teams[3]);
                    })
                });
            });
    }
}
