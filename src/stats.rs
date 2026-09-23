use serde::Deserialize;

const K: f32 = 20.0;

#[derive(Debug)]
pub struct Stats {
    pub games_played: u32,
    pub wins: u32,
    pub losses: u32,
}

impl Stats {
    pub fn get_skill(&self) -> u32 {
        let g = self.games_played as f32;
        let w = self.wins as f32;

        let p = (w + 0.25 * K) / (g + K);
        
        ((p * 1000.0) as u32).clamp(0, 1000)
    }
}

#[derive(Debug, Deserialize)]
pub struct RawStats {
    #[serde(rename = "Games played")]
    games_played: Stat,
    #[serde(rename = "Wins")]
    wins: Stat,
    #[serde(rename = "Losses")]
    losses: Stat,
}

#[derive(Debug, Deserialize)]
struct Stat {
    #[serde(default, deserialize_with = "null_as_empty_vec")]
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
struct Entry {
    value: String,
}

impl From<RawStats> for Stats {
    fn from(raw: RawStats) -> Self {
        Self {
            games_played: first_value(raw.games_played),

            wins: first_value(raw.wins),
            losses: first_value(raw.losses),
        }
    }
}

fn first_value(stat: Stat) -> u32 {
    stat.entries
        .first()
        .map(|entry| entry.value.parse().unwrap_or(0))
        .unwrap_or(0)
}

fn null_as_empty_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<Vec<T>>::deserialize(deserializer).map(Option::unwrap_or_default)
}
