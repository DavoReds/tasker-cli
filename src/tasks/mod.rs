pub mod application;
use std::io::{BufReader, Read};

use self::application::app_directory;

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    name: String,
    done: bool,
}

impl Task {
    /// Returns a new Task with name `name`, marked as not done.
    pub fn new(name: String) -> Self {
        Self { name, done: false }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Todo {
    pub tasks: Vec<Task>,
}

impl Todo {
    /// Reads content of `.tasker/tasks.yml` file. If it doesn't exist, it
    /// creates it and initializes it with an empty vector of tasks
    pub fn new() -> Result<Self> {
        let file_path = app_directory()
            .expect("Failed to read app directory")
            .join("tasks.yml");

        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(file_path)?;

        let mut reader = BufReader::new(file);
        let mut content = String::new();

        match reader.read_to_string(&mut content) {
            Ok(_) => Ok(serde_yaml::from_str(&content).unwrap_or_else(|_| Todo::default())),
            Err(e) => Err(anyhow!("{}", e)),
        }
    }

    /// Adds a new task to the Todo object.
    pub fn add_task(&mut self, name: String) -> Result<()> {
        self.tasks.push(Task::new(name));

        self.save()?;

        Ok(())
    }

    /// Saves current Todo object to a file in the application directory.
    fn save(&self) -> Result<()> {
        let file_path = app_directory()
            .expect("Failed to read app directory")
            .join("tasks.yml");

        std::fs::write(file_path, serde_yaml::to_string(&self)?)
            .context("Failed to write tasks file")?;

        Ok(())
    }
}
