use std::path::PathBuf;

use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use super::Todo;
use crate::{
    config::{Config, Language},
    Cli, Command,
};

/// Function that runs the entire program. It pattern matches agains the
/// command option and performs the appropiate function for each subcommand.
pub fn tasker_run(config: &Config, args: &Cli, mut todo: Todo) -> Result<()> {
    match &args.command {
        Command::Config(cfg) => {
            Config::write_config(cfg, todo).context("Failed to change configuration.")?;

            match cfg.language {
                Language::English => {
                    println!("{}", "Configuration updated!".green());
                }
                Language::Spanish => {
                    println!("{}", "¡Configuración actualizada!".green());
                }
            }
        }

        Command::Create(task) => {
            todo.add_task(task.task.clone())
                .context("Failed to create task.")?;

            match config.language {
                Language::English => {
                    println!("Task: {} created", task.task.purple());
                }
                Language::Spanish => {
                    println!("Tarea: {} creada", task.task.purple());
                }
            }
        }

        Command::Complete(task) => {
            let mut completed_task = todo.tasks.get_mut(task.id).context("No such task exists")?;
            completed_task.done = true;

            let completed_task = completed_task.name.clone();

            todo.save().context("Failed to save tasks.yml file")?;

            match config.language {
                Language::English => {
                    println!("Task: {} completed", completed_task.green());
                }
                Language::Spanish => {
                    println!("Tarea: {} completada", completed_task.green());
                }
            }
        }

        Command::Delete(task) => {
            let deleted_task = todo.tasks.remove(task.id).name;

            todo.save().context("Failed to save tasks.yml file")?;

            match config.language {
                Language::English => {
                    println!("Task: {} deleted", deleted_task.red());
                }
                Language::Spanish => {
                    println!("Tarea: {} eliminada", deleted_task.red());
                }
            }
        }

        Command::Edit(task) => {
            let mut edited_task = todo.tasks.get_mut(task.id).context("No such task exists")?;
            edited_task.name = task.task.clone();

            todo.save().context("Failed to save tasks.yml file")?;

            match config.language {
                Language::English => {
                    println!("Task: {} edited", task.task.magenta());
                }
                Language::Spanish => {
                    println!("Tarea: {} editada", task.task.magenta());
                }
            }
        }

        Command::List => match config.language {
            Language::English => {
                println!(
                    "Good day, {}.\nHere's what you got for today!\n\n{}",
                    config.name.magenta(),
                    todo
                );
            }
            Language::Spanish => {
                println!(
                    "Buen día, {}.\n¡Esto es lo que tienes para hoy!\n\n{}",
                    config.name.magenta(),
                    todo
                )
            }
        },

        Command::Clean => {
            todo.clean_tasks().context("Failed to clean tasks.")?;

            match config.language {
                Language::English => {
                    println!("{}", "Cleaned completed tasks!".green());
                }
                Language::Spanish => {
                    println!("{}", "¡Se limpiaron las tareas completadas!".green());
                }
            }
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
