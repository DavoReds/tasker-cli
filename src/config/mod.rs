use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub language: Language,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Language {
    English,
    Spanish,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: "John Doe".into(),
            language: Language::English,
        }
    }
}

impl Config {
    pub fn load_config() -> Result<Config, confy::ConfyError> {
        let cfg: Config = confy::load("tasker", "tasker_cli")?;

        Ok(cfg)
    }
}
