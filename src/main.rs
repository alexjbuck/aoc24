mod cli;
mod commands;
mod runner;
mod tui;

use clap::Parser;
use cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path } => commands::init::execute(path)?,
        Commands::Run { day, part, input } => commands::run::execute(day, part, input)?,
        Commands::Watch => commands::watch::execute()?,
    }

    Ok(())
}
