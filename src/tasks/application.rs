use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::{config::Config, Cli, Command};

use super::Todo;

/// Function that runs the entire program. It pattern matches agains the
/// command option and performs the appropiate function for each subcommand.
pub fn tasker_run(_config: &Config, args: &Cli, mut todo: Todo) -> Result<()> {
    match &args.command {
        Command::Create(task) => {
            todo.add_task(task.task.clone())?;
        }

        Command::Config(cfg) => {
            Config::write_config(cfg)?;
        }

        Command::Complete(task) => {
            todo.tasks[task.id].done = true;

            todo.save().context("Failed to save tasks.yml file")?;
        }
        _ => (),
    }

    Ok(())
}

/// Returns path to the program folder. This folder is where all tasks are
/// saved.
pub fn app_directory() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".tasker"))
}

/// Creates .tasker directory in user folder.
///
/// # Errors
///
/// The function returns an error if the client is not using Windows, MacOS or
/// Linux.
pub fn create_app_directory() -> Result<()> {
    let directory = app_directory().context("Not using a supported OS")?;

    std::fs::create_dir_all(directory).context("Error creating .tasker directory")?;

    Ok(())
}
