// main.rs
use crate::kraken::Kraken;
use anyhow::Result;
use clap::Parser;
use execute::Execute;

mod add;
mod execute;
mod kraken;
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Kraken,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.command.execute()
}
