#![allow(dead_code)]

mod config;
mod receive;
mod request;
mod send;
mod server;
mod utils;

use clap::{Parser, Subcommand};
use config::ConfigOnceCell;

pub static CONFIG: ConfigOnceCell = ConfigOnceCell::new();

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
    Test,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Send(args) => send::send(args).await,
        Commands::Receive(args) => receive::receive(args).await,
        Commands::Test => {
            utils::select::select_test().await.unwrap();
            Ok(())
        }
    };
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}
