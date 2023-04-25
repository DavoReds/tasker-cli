use clap::{Args, Parser, Subcommand};

use crate::config::Language;

/// A To-Do CLI application for managing your daily tasks
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// Subcommand for the application
#[derive(Subcommand, Debug)]
pub enum Command {
    Create(CreateTask),
    Edit(EditTask),
    Mark(MarkTask),
    Delete(DeleteTask),

    /// Cleans all completed Tasks
    Clean,

    /// Lists all Tasks
    List,

    Config(ConfigApp),
}

/// Creates a new Task
#[derive(Args, Debug)]
pub struct CreateTask {
    /// Name of the Task
    pub task: String,
}

/// Edits an existing Task
#[derive(Args, Debug)]
pub struct EditTask {
    /// ID of the Task to edit
    pub id: usize,

    /// New body for the task
    pub task: String,
}

/// Toggles the completion of a Task
#[derive(Args, Debug)]
pub struct MarkTask {
    /// ID of the Task to toggle
    pub id: usize,
}

/// Deletes a Task, whether completed or not
#[derive(Args, Debug)]
pub struct DeleteTask {
    /// ID of the Task to delete
    pub id: usize,
}

/// Configures the application
#[derive(Args, Debug)]
pub struct ConfigApp {
    /// Name of the user
    #[arg(short, long)]
    pub name: String,

    /// Language for the program to use
    #[arg(value_enum, short, long)]
    pub language: Language,
}
