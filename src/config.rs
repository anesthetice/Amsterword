use serde::{
    Serialize,
    Deserialize
};
use serde_json;
use std::{
    io,
    fs::read,
};
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub number_of_players: usize,
    pub hint_system: bool,
    pub hint_probability_anomaly: f64,
    pub hint_probability_generic: f64,
    pub lexicon: Vec<String>,
}

impl Config {
    pub fn load_from_file<P: AsRef<std::path::Path>>(filepath: P) -> io::Result<Self> {
        Ok(serde_json::from_slice(&read(filepath)?)?)
    }
}