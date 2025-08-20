use crate::cli::{Cli, Commands};
use clap::Parser;
mod types;
mod cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { ssid, password, security, output, size, hidden} => {
            //TODO: Input Validation
        }
    }
}
