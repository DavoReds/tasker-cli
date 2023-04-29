use anyhow::{bail, Ok, Result};
use clap::Parser;
use tasker_cli::{
    config::Config,
    tasker_run,
    tasks::{application::create_app_directory, Todo},
    Cli,
};

fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Create .tasker directory if it doesn't exist already.
    // Return the program with an error if it fails.
    if let Err(e) = create_app_directory() {
        bail!("Error creating program directory: {}", e);
    }

    if let tasker_cli::Command::Config(ref cfg) = cli.command {
        Config::write_config(cfg)?;
    }

    let config = Config::load_config()?;

    let todo: Todo = Todo::new();

    tasker_run(&config, &cli, todo)?;

    Ok(())
}
