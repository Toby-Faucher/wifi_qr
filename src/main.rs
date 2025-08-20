use clap::{Parser, Subcommand};

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
        ssid: String

    }
}
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { ssid } => {
            println!("Test with {}", ssid);
        }
    }
}
