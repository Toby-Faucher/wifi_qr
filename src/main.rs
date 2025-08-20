mod cli;
mod types;
mod qr;
use crate::cli::{Cli, Commands};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { ssid, password, security, output, size, hidden} => {
            //TODO: Input Validation
            todo!()
        }
    }
}
