use serde::Deserialize;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub name: String,
}

impl Config {
    pub fn new(path: PathBuf) -> io::Result<Config> {
        let config = fs::read_to_string(path)?;

        // You have a String, and need to convert it to &str, referencing the String with &
        // already converts it to a &str
        let config: Config = toml::from_str(&config)?;

        // Returns? Where we're going, we don't need returns.
        Ok(config)
    }
}
