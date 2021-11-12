use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct GhRepo {
    pub repo: String,
    pub users : Vec<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct GhConfig {
    pub token: String,
    pub repos: Vec<GhRepo>,
}