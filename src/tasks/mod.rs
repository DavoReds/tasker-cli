pub mod application;
use self::application::app_directory;

use anyhow::{Context, Ok, Result};
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub tasks: Vec<Task>,
}

impl Todo {
    /// Returns a new Todo object, holding an empty Vector.
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
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

impl Default for Todo {
    fn default() -> Self {
        Self::new()
    }
}
