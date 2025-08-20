mod cli;
mod types;
mod qr;
use crate::cli::{Cli, Commands};
use crate::qr::{WifiQr, gen_qr};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { ssid, password, security, output, size, hidden, terminal} => {
            let wifi_struct = WifiQr::new(ssid, password, security, size, hidden);
            match wifi_struct {
                Ok(wifi_qr) => {
                    if let Err(e) = gen_qr(wifi_qr, output, Some(terminal)) {
                        eprintln!("Error generating QR code: {}", e);
                    }
                }
                Err(e) => eprintln!("Error creating WiFi QR: {:?}", e),
            }
        }
    }
}
