mod receive;
mod send;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None, styles = utils::clap::clap_styles())]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Send(send::SendArgs),
    Receive(receive::ReceiveArgs),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Send(args) => send::send(args),
        Commands::Receive(args) => receive::receive(args),
    }

    Ok(())
}
