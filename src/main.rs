use anyhow::{Ok, Result};
use clap::Parser;
use tasker_cli::{config::Config, Cli};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let cfg = Config::load_config()?;

    dbg!(cli);
    dbg!(&cfg);

    println!("Hi, {}!", cfg.name);

    Ok(())
}
