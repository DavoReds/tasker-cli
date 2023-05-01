use anyhow::{bail, Ok, Result};
use clap::Parser;
use tasker_cli::{config::Config, create_app_directory, tasker_run, tasks::Todo, Cli};

fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Create .tasker directory if it doesn't exist already.
    // Return the program with an error if it fails.
    if let Err(e) = create_app_directory() {
        bail!("Error creating program directory: {}", e);
    }

    // Load program's configuration
    let config = Config::load_config()?;

    // Read contents of tasks.yml file into a Todo object, or create a new one
    // if no such file exists
    let todo: Todo = Todo::new()?;

    // Run the application
    tasker_run(&config, &cli, todo)?;

    Ok(())
}
