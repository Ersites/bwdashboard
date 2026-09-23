use std::thread;

use eframe::egui;
use log::error;
use thiserror::Error;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::{TcpListener, TcpStream},
    runtime::Builder,
    sync::mpsc,
};

use crate::{
    event::Event,
    parser,
    protocol::Message,
    stats::{RawStats, Stats},
};

pub fn start(
    ready_tx: crossbeam::channel::Sender<mpsc::Sender<ClientEvent>>,
    event_tx: crossbeam::channel::Sender<Event>,
    ctx: egui::Context,
) {
    thread::spawn(move || {
        let rt = Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime");

        rt.block_on(async {
            let listener = TcpListener::bind("127.0.0.1:52138")
                .await
                .expect("Failed to create TCP Listener");

            let (client_tx, mut client_rx) = mpsc::channel(100);
            ready_tx.send(client_tx.clone()).unwrap();

            let request_client = reqwest::Client::new();

            let mut client_connected = false;

            loop {
                tokio::select! {
                    Ok((stream, _)) = listener.accept(), if !client_connected => {
                        client_connected = true;
                        let event_tx = event_tx.clone();
                        let ctx = ctx.clone();
                        tokio::spawn(
                            handle_client(stream, client_tx.clone(), event_tx, ctx)
                        );
                    }
                    Some(event) = client_rx.recv() => {
                        match event {
                            ClientEvent::Disconnected => {
                                client_connected = false;
                            }
                            ClientEvent::GetPlayerSkill(name) => {
                                let request_client = request_client.clone();
                                let event_tx = event_tx.clone();
                                let ctx = ctx.clone();
                                tokio::spawn(async {
                                    if let Err(e) = fetch_stats(name, request_client, event_tx, ctx).await {
                                        error!("fetch_stats failed: {e}");
                                    }
                                } );
                            }
                        }
                    }
                }
                ctx.request_repaint();
            }
        });
    });
}

async fn handle_client(
    stream: TcpStream,
    client_tx: mpsc::Sender<ClientEvent>,
    event_tx: crossbeam::channel::Sender<Event>,
    ctx: egui::Context,
) {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    loop {
        line.clear();

        if let Ok(bytes) = reader.read_line(&mut line).await {
            if bytes == 0 {
                break;
            }

            parse_line(&line, &event_tx, ctx.clone());
        }
    }

    event_tx.send(Event::Disconnected).unwrap();
    client_tx.send(ClientEvent::Disconnected).await.unwrap();
}

fn parse_line(line: &str, event_tx: &crossbeam::channel::Sender<Event>, ctx: egui::Context) {
    match serde_json::from_str::<Message>(line) {
        Ok(message) => {
            for event in parser::parse(message) {
                event_tx.send(event).unwrap();
                ctx.request_repaint();
            }
        }
        Err(e) => error!("Line parsing error: {}", e),
    };
}

pub enum ClientEvent {
    Disconnected,
    GetPlayerSkill(String),
}

impl ClientEvent {
    pub fn get_player_skill(name: &str) -> Self {
        ClientEvent::GetPlayerSkill(name.to_owned())
    }
}

async fn fetch_stats(
    name: String,
    client: reqwest::Client,
    event_tx: crossbeam::channel::Sender<Event>,
    ctx: egui::Context,
) -> Result<(), FetchStatsError> {
    let url = format_url(&name);
    let response = client.get(url).send().await?.error_for_status()?;
    let body = response.text().await?;
    let raw_stats: RawStats = serde_json::from_str(&body).map_err(|err| FetchStatsError::deserialize(err, body))?;
    let stats: Stats = raw_stats.into();
    event_tx.send(Event::set_player_skill(name, stats.get_skill())).unwrap();
    ctx.request_repaint();
    Ok(())
}

fn format_url(name: &str) -> String {
    format!(
        "https://stats.pika-network.net/api/profile/{}/leaderboard?type=bedwars&interval=total&mode=QUAD",
        name
    )
}

#[derive(Debug, Error)]
pub enum FetchStatsError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Failed to deserialize response: {source}\nJSON body: {body}")]
    Deserialize {
        source: serde_json::Error,
        body: String,
    },
}

impl FetchStatsError {
    fn deserialize(source: serde_json::Error, body: String) -> Self {
        Self::Deserialize { source, body }
    }
}
