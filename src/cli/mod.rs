use clap::{Args, Parser, Subcommand};

use crate::config::Language;

/// A To-Do CLI application for managing your daily tasks
#[derive(Parser, Debug)]
#[command(
    name = "Tasker CLI",
    author,
    version,
    about,
    long_about = None,
    help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

/// Subcommands for the application
#[derive(Subcommand, Debug, Default)]
#[command(help_template(
    "\
{name}
{about-with-newline}
{usage-heading} {usage}

{all-args}"
))]
pub enum Command {
    /// Creates a new task
    #[command(visible_alias = "cr")]
    #[command(arg_required_else_help = true)]
    Create(CreateTask),

    /// Edits an existing task
    #[command(visible_alias = "e")]
    #[command(arg_required_else_help = true)]
    Edit(EditTask),

    /// Marks a task as complete
    #[command(visible_alias = "co")]
    #[command(arg_required_else_help = true)]
    Complete(CompleteTask),

    /// Deletes a task, whether completed or not
    #[command(visible_alias = "d")]
    #[command(arg_required_else_help = true)]
    Delete(DeleteTask),

    /// Cleans all completed tasks
    #[command(visible_alias = "cl")]
    #[command(help_template(
        "\
{name}
{about-with-newline}
{usage-heading} {usage}

{all-args}"
    ))]
    Clean,

    /// Lists all tasks [default]
    #[default]
    #[command(alias = "ls")]
    #[command(short_flag = 'l')]
    #[command(help_template(
        "\
{name}
{about-with-newline}
{usage-heading} {usage}

{all-args}"
    ))]
    List,

    /// Configures the application
    #[command(visible_alias = "cfg")]
    #[command(arg_required_else_help = true)]
    Config(ConfigApp),

    /// Get path to the configuration file
    #[command(short_flag = 'p')]
    Path,
}

#[derive(Args, Debug)]
#[command(help_template(
    "\
{name}
{about-with-newline}
{usage-heading} {usage}

{all-args}"
))]
pub struct CreateTask {
    /// Name of the task(s) separated by commas. Wrap in quotes for multi word
    /// tasks
    pub tasks: Vec<String>,
}

#[derive(Args, Debug)]
#[command(help_template(
    "\
{name}
{about-with-newline}
{usage-heading} {usage}

{all-args}"
))]
pub struct EditTask {
    /// ID of the task to edit
    pub id: usize,

    /// New body for the task
    pub task: String,
}

#[derive(Args, Debug)]
#[command(help_template(
    "\
{name}
{about-with-newline}
{usage-heading} {usage}

{all-args}"
))]
pub struct CompleteTask {
    /// ID of the task(s) to mark as complete
    pub id: Vec<usize>,
}

#[derive(Args, Debug)]
#[command(help_template(
    "\
{name}
{about-with-newline}
{usage-heading} {usage}

{all-args}"
))]
pub struct DeleteTask {
    /// ID of the task(s) to delete
    pub id: Vec<usize>,
}

#[derive(Args, Debug)]
#[command(help_template(
    "\
{name}
{about-with-newline}
{usage-heading} {usage}

{all-args}"
))]
pub struct ConfigApp {
    /// Name of the user
    #[arg(short, long)]
    pub name: String,

    /// Language for the program to use
    #[arg(value_enum, short, long)]
    pub language: Language,
}
