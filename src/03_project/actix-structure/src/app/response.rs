use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Response<T> {
    pub msg: String,
    pub data: Option<T>,
}

#[derive(Deserialize, Serialize)]
pub struct GenernalError {
    pub msg: String,
    pub detail: Option<String>,
}