use anyhow::{Ok, Result};
use clap::Parser;
use tasker_cli::{config::Config, Cli};

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let tasker_cli::Command::Config(cfg) = cli.command {
        Config::write_config(cfg)?;
    }

    let cfg = Config::load_config()?;

    println!("Hi, {}!", cfg.name);

    Ok(())
}
