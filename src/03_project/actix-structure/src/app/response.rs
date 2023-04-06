use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GenernalError {
    pub msg: String,
    pub detail: Option<String>,
}