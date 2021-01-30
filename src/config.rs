use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Party {
    pub host: String,
    pub join: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub parties: Vec<Party>,
}

impl Config {
    pub fn load(path: &Path) -> Result<Config,String> {
        match std::fs::read(path) {
            Ok(bs) => match serde_json::from_str(std::str::from_utf8(&bs).expect("Not utf8?")) {
                Ok(c) => Ok(c),
                Err(e) => Err(format!("{}", e)),
            },
            Err(e) => Err(format!("{}", e)),
        }
    }
}
