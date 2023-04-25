use anyhow::Result;

use crate::{config::Config, Cli, Command};

use super::Task;

pub fn tasker_run(_config: &Config, args: &Cli) -> Result<()> {
    match &args.command {
        Command::Create(task) => {
            let task = Task::new(task.task.clone());
            task.create_task()?;
        }
        _ => (),
    }

    Ok(())
}
