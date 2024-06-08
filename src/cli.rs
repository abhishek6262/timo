use clap::Parser;

use crate::commands::Commands;

#[derive(Parser)]
#[command(version, about = "Capture and search thoughts in your terminal.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
