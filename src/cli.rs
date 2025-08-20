use clap::{Parser, Subcommand};
use crate::types::SecurityType;

#[derive(Parser)]
#[command(name = "wifi-qr")]
#[command(about = "Generates Commands For WIFI Networks")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    pub command: Commands
}


#[derive(Subcommand)]
pub enum Commands {
    Generate {

        #[arg(short,long)]
        ssid: String,

        #[arg(short,long)]
        password: Option<String>,

        #[arg(short = 't', long, default_value = "wpa2")]
        security: SecurityType,

        //TODO: Make file path type (or find one)
        #[arg(short,long)]
        output: Option<String>,
        
        #[arg(long,default_value = "256")]
        size: u32,

        #[arg(long)]
        hidden: bool,
    }
}
