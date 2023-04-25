pub mod application;

use std::{fs::File, io::Write, path::PathBuf};

use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    name: String,
    done: bool,
}

impl Task {
    pub fn new(name: String) -> Self {
        Self { name, done: false }
    }

    pub fn create_task(&self) -> Result<()> {
        let mut path = PathBuf::new();
        path.push("/home/dalia/");
        path.push(format!("{}.yml", self.name));

        let contents = serde_yaml::to_string(self)?;

        let mut file = File::create(path)?;
        file.write_all(contents.as_bytes())?;

        Ok(())
    }
}
