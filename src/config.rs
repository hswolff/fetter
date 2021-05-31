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
        let config: Config = toml::from_str(config.as_str())?;

        return Ok(config);
    }
}
