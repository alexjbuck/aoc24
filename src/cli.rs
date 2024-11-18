use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Advent of Code workspace
    Init {
        /// Path where to create the workspace
        path: PathBuf,
    },
    /// Run a specific day and part
    Run {
        /// Day number (1-25)
        day: u8,
        /// Part number (1-2)
        part: u8,
        /// Optional input file path
        #[arg(short, long)]
        input: Option<PathBuf>,
    },
    /// Watch mode with TUI
    Watch,
}
