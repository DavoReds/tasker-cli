pub mod application;
use std::{
    fmt::Display,
    io::{BufReader, Read},
};

use crate::config::Language;

use self::application::app_directory;

use anyhow::{anyhow, Context, Result};
use owo_colors::OwoColorize;
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

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.done {
            return write!(f, "{}", self.name.purple());
        }

        write!(f, "{}", self.name.purple())
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Todo {
    pub tasks: Vec<Task>,
    pub language: Language,
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
            .open(file_path)
            .context("Failed to read app directory contents.")?;

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

        Ok(())
    }

    /// Saves current Todo object to a file in the application directory.
    pub fn save(&self) -> Result<()> {
        let file_path = app_directory()
            .expect("Failed to read app directory")
            .join("tasks.yml");

        std::fs::write(file_path, serde_yaml::to_string(&self)?)
            .context("Failed to write tasks file")?;

        Ok(())
    }

    pub fn clean_tasks(&mut self) -> Result<()> {
        self.tasks = self
            .tasks
            .clone()
            .into_iter()
            .filter(|task| !task.done)
            .collect();

        self.save().context("Failed to save tasks.yml file")?;

        Ok(())
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tasks = String::new();

        for (id, task) in self.tasks.iter().enumerate() {
            tasks.push_str(&format!("({}): ", id.blue()));
            tasks.push_str(&format!("{}\n", task));

            match self.language {
                Language::English => match task.done {
                    true => {
                        tasks.push_str(&format!("[{}]", "Done".green()));
                    }
                    false => {
                        tasks.push_str(&format!("[{}]", "To Do".red()));
                    }
                },
                Language::Spanish => match task.done {
                    true => {
                        tasks.push_str(&format!("[{}]", "Hecho".green()));
                    }
                    false => {
                        tasks.push_str(&format!("[{}]", "Por Hacer".red()));
                    }
                },
            }

            tasks.push('\n');
            tasks.push('\n');
        }

        write!(f, "{}", tasks)
    }
}
