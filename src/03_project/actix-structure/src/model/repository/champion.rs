use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Champion {
    pub name: String,
    pub role: String,
    pub health: i32,
    pub affiliation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChampionOut {
    pub name: String,
}