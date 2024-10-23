#![allow(dead_code)]

mod config;
mod receive;
mod request;
mod send;
pub mod utils;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, long_about = None, styles = utils::clap::clap_styles())]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Send(send::SendArgs),
    Receive(receive::ReceiveArgs),
    // Test,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut config = config::Config::new();

    match cli.command {
        Commands::Send(args) => send::send(args, &mut config).await?,
        Commands::Receive(args) => receive::receive(args, &mut config).await?,
        // Commands::Test => utils::multiselect::metadata_select().await?,
    }

    Ok(())
}
