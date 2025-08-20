use clap::{Parser, Command};

#[derive(Parser)]
struct Cli {}

fn main() {
    let _cli = Cli::parse();
    println!("Hello World");
}
