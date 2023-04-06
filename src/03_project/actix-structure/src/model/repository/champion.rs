use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)];
pub struct Champion {
    name: String,
    role: String,
    health: i32,
    affiliation: String,
}

#[derive(Debug, Serialize, Deserialize)];
pub struct ChampionOut {
    name: String,
}