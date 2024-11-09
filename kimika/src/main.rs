#![allow(dead_code)]

mod config;
mod receive;
mod request;
mod send;
mod server;
mod utils;

use clap::{Parser, Subcommand};
use config::ConfigOnceCell;
use once_cell::sync::OnceCell;

pub static CONFIG: ConfigOnceCell = ConfigOnceCell(OnceCell::new());

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
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Send(args) => send::send(args).await,
        Commands::Receive(args) => receive::receive(args).await,
        // Commands::Test => utils::multiselect::metadata_select().await?,
    };
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}
