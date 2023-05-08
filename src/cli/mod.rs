use clap::{Args, Parser, Subcommand};

use crate::config::Language;

/// A To-Do CLI application for managing your daily tasks
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

/// Subcommands for the application
#[derive(Subcommand, Debug, Default)]
pub enum Command {
    /// Creates a new task
    Create(CreateTask),

    /// Edits an existing task
    Edit(EditTask),

    /// Marks a task as complete
    Complete(CompleteTask),

    /// Deletes a task, whether completed or not
    Delete(DeleteTask),

    /// Cleans all completed tasks
    Clean,

    /// Lists all tasks (Default)
    #[default]
    List,

    /// Configures the application
    Config(ConfigApp),

    /// Get path to the configuration file
    Path,
}

#[derive(Args, Debug)]
pub struct CreateTask {
    /// Name of the task
    pub tasks: Vec<String>,
}

#[derive(Args, Debug)]
pub struct EditTask {
    /// ID of the task to edit
    pub id: usize,

    /// New body for the task
    pub task: String,
}

#[derive(Args, Debug)]
pub struct CompleteTask {
    /// ID of the task to toggle
    pub id: Vec<usize>,
}

#[derive(Args, Debug)]
pub struct DeleteTask {
    /// ID of the task to delete
    pub id: Vec<usize>,
}

#[derive(Args, Debug)]
pub struct ConfigApp {
    /// Name of the user
    #[arg(short, long)]
    pub name: String,

    /// Language for the program to use
    #[arg(value_enum, short, long)]
    pub language: Language,
}
