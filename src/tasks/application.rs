use std::path::PathBuf;

use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use super::Todo;
use crate::{config::Config, Cli, Command};

/// Function that runs the entire program. It pattern matches agains the
/// command option and performs the appropiate function for each subcommand.
pub fn tasker_run(config: &Config, args: &Cli, mut todo: Todo) -> Result<()> {
    match &args.command {
        Command::Config(cfg) => {
            Config::write_config(cfg).context("Failed to change configuration.")?;
        }

        Command::Create(task) => {
            todo.add_task(task.task.clone())
                .context("Failed to create task.")?;
        }

        Command::Complete(task) => {
            todo.tasks
                .get_mut(task.id)
                .context("No such task exists")?
                .done = true;

            todo.save().context("Failed to save tasks.yml file")?;
        }

        Command::Delete(task) => {
            todo.tasks.remove(task.id);

            todo.save().context("Failed to save tasks.yml file")?;
        }

        Command::Edit(task) => {
            todo.tasks
                .get_mut(task.id)
                .context("No such task exists")?
                .name = task.task.clone();

            todo.save().context("Failed to save tasks.yml file")?;
        }

        Command::List => {
            println!(
                "Good day, {}.\nHere's what you got for today!\n\n{}",
                config.name.magenta(),
                todo
            );
        }

        Command::Clean => {
            todo.clean_tasks().context("Failed to clean tasks.")?;
        }
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
