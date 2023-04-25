use anyhow::{Ok, Result};
use clap::Parser;
use tasker_cli::{config::Config, tasker_run, Cli};

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let tasker_cli::Command::Config(ref cfg) = cli.command {
        Config::write_config(cfg)?;
    }

    let config = Config::load_config()?;

    tasker_run(&config, &cli)?;

    Ok(())
}
