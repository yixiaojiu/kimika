mod local;
mod remote;

use crate::CONFIG;
use clap::Args;
use crossterm::{style::Stylize, tty::IsTty};

/// Send file or message
#[derive(Args, Debug)]
#[command(version, long_about = None)]
pub struct SendArgs {
    /// The path of file which want to send
    #[arg(short, long, value_name = "path")]
    pub path: Option<Vec<std::path::PathBuf>>,

    /// Text which wants to send
    #[arg(short, long, value_name = "message")]
    pub message: Option<Vec<String>>,

    /// Receiver address or remote server address. Such as: example.com:3939
    #[arg(short, long, value_name = "address")]
    pub address: Option<String>,

    /// Listen port
    #[arg(long, value_name = "port")]
    pub port: Option<u16>,

    /// Receiver listening port when transfer from local network
    #[arg(long, value_name = "receiver_port")]
    pub receiver_port: Option<u16>,

    /// Alias used for identification
    #[arg(long, value_name = "alias")]
    pub alias: Option<String>,

    /// Whether to use remote server
    #[arg(short, long, value_name = "server", default_value = "false")]
    pub server: bool,
}

pub async fn send(args: SendArgs) -> Result<(), Box<dyn std::error::Error>> {
    if args.path.is_none() && args.message.is_none() && std::io::stdin().is_tty() {
        println!("{}", "Please specify a file or a message".yellow());
        return Ok(());
    }

    CONFIG.set_from_send_args(&args).unwrap();

    if args.server {
        remote::remote_send(&args).await?;
    } else {
        local::local_send(&args).await?;
    }

    Ok(())
}
