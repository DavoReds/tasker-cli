use std::path::PathBuf;

use anyhow::{Context, Result};
use owo_colors::OwoColorize;

use super::{Task, Todo};
use crate::{
    cli::Command,
    config::{Config, Language},
    Cli,
};

/// Function that runs the entire program. It pattern matches agains the
/// command option and performs the appropiate function for each subcommand.
pub fn tasker_run(config: &Config, args: &mut Cli, mut todo: Todo) -> Result<()> {
    if args.command.is_none() {
        args.command = Some(Command::default());
    }
    match &args.command.as_ref().context("No argument given")? {
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

        Command::Create(tasks) => {
            for task in &tasks.tasks {
                todo.add_task(task.clone());

                match config.language {
                    Language::English => {
                        println!("Task: {} created", task.purple());
                    }
                    Language::Spanish => {
                        println!("Tarea: {} creada", task.purple());
                    }
                }
            }
            todo.save().context("Failed to write tasks file")?;
        }

        Command::Complete(tasks) => {
            for task in &tasks.id {
                let completed_task = todo
                    .tasks
                    .get_mut(*task)
                    .context(format!("Task {task} doesn't exist"))?;
                completed_task.done = true;

                let completed_task = completed_task.name.clone();

                match config.language {
                    Language::English => {
                        println!("Task: {} completed", completed_task.green());
                    }
                    Language::Spanish => {
                        println!("Tarea: {} completada", completed_task.green());
                    }
                }
            }

            todo.save().context("Failed to save tasks.yml file")?;
        }

        Command::Delete(tasks) => {
            let deleted_tasks: Vec<Task> = tasks
                .id
                .iter()
                .filter_map(|idx| todo.tasks.get(*idx).cloned())
                .collect();

            todo.tasks.retain(|task| !deleted_tasks.contains(task));

            let mut deleted_ids: Vec<String> = Vec::new();
            for id in &tasks.id {
                deleted_ids.push(id.to_string());
            }
            let deleted_tasks = deleted_ids.join(", ");

            match config.language {
                Language::English => {
                    if deleted_ids.len() > 1 {
                        println!("Tasks: {} deleted", deleted_tasks.red());
                    } else {
                        println!("Task: {} deleted", deleted_tasks.red());
                    }
                }
                Language::Spanish => {
                    if deleted_ids.len() > 1 {
                        println!("Tareas: {} eliminadas", deleted_tasks.red());
                    } else {
                        println!("Tarea: {} eliminada", deleted_tasks.red());
                    }
                }
            }
            todo.save().context("Failed to save tasks.yml file")?;
        }

        Command::Edit(task) => {
            let edited_task = todo
                .tasks
                .get_mut(task.id)
                .context(format!("Task {} doesn't exist", task.id))?;
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
                print!(
                    "Good day, {}.\nHere's what you got for today!\n\n{}",
                    config.name.magenta(),
                    todo
                );
            }
            Language::Spanish => {
                print!(
                    "Buen día, {}.\n¡Esto es lo que tienes para hoy!\n\n{}",
                    config.name.magenta(),
                    todo
                );
            }
        },

        Command::Clean => {
            todo.tasks.retain(|task| !task.done);

            todo.save().context("Failed to save tasks.yml file")?;

            match config.language {
                Language::English => {
                    println!("{}", "Cleaned completed tasks!".green());
                }
                Language::Spanish => {
                    println!("{}", "¡Se limpiaron las tareas completadas!".green());
                }
            }
        }

        Command::Path => {
            println!(
                "{}",
                confy::get_configuration_file_path("tasker", "tasker_cli")
                    .context("Failed to get configuration path")?
                    .to_str()
                    .context("Failed to parse configuration path")?
            );
        }
    }

    Ok(())
}

/// Returns path to the folder in which all tasks are saved.
pub fn app_directory() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".tasker"))
}

/// Creates `.tasker` directory in user folder.
///
/// # Errors
///
/// The function returns an error if the client is not using `Windows`, `MacOS`
/// or `Linux`.
pub fn create_app_directory() -> Result<()> {
    let directory = app_directory().context("Not using a supported OS")?;

    std::fs::create_dir_all(directory).context("Error creating .tasker directory")?;

    Ok(())
}
