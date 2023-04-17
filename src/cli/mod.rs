use clap::{Args, Parser, Subcommand};

/// A To-Do CLI application for managing your daily tasks
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Create(CreateTask),
    Edit(EditTask),
    Mark(MarkTask),
    Delete(DeleteTask),

    /// Cleans all completed Tasks
    Clean,

    /// Lists all Tasks
    List,
}

/// Creates a new Task
#[derive(Args)]
pub struct CreateTask {
    /// Name of the Task
    pub task: String,
}

/// Edits an existing Task
#[derive(Args)]
pub struct EditTask {
    /// ID of the Task to edit
    pub id: usize,

    /// New body for the task
    pub task: String,
}

/// Toggles the completion of a Task
#[derive(Args)]
pub struct MarkTask {
    /// ID of the Task to toggle
    pub id: usize,
}

/// Deletes a Task, whether completed or not
#[derive(Args)]
pub struct DeleteTask {
    /// ID of the Task to delete
    pub id: usize,
}
