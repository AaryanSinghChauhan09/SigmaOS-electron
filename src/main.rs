#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "sigmaos-electron", about = "Electron desktop web application runtime for SigmaOS")]
struct Cli { #[command(subcommand)] cmd: Commands }
#[derive(Subcommand)]
enum Commands { List, Status }
fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List   => println!("sigmaos-electron: no entries configured."),
        Commands::Status => println!("sigmaos-electron: ready."),
    }
}
