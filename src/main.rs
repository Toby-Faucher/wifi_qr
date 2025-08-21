mod cli;
mod types;
mod config;
mod qr;
use crate::cli::{Cli, Commands};
use crate::qr::{WifiQr, gen_qr};
use crate::config::WifiQrConfig;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    
    // Initialize config - creates default if missing
    let _config = match WifiQrConfig::load_or_create() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            std::process::exit(1);
        }
    };

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
