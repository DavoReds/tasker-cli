use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::default::Default;

use crate::ConfigApp;

/// # Configuration structure for the program
///
/// **Name:** Name of the user.
/// **Language:** The language the program will use.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub language: Language,
}

/// # Language
///
/// The language the entire application will use to communicate with the user.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialOrd, Ord, PartialEq, ValueEnum)]
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
    /// Loads the program's configuration file.
    pub fn load_config() -> Result<Config, confy::ConfyError> {
        // It's abstracted away to ensure all three versions of the program
        // use the same config folder
        let cfg: Config = confy::load("tasker", "tasker_cli")?;

        Ok(cfg)
    }

    pub fn write_config(config: ConfigApp) -> anyhow::Result<()> {
        let configuration = Config {
            name: config.name,
            language: config.language,
        };

        confy::store("tasker", "tasker_cli", configuration)?;

        Ok(())
    }
}
