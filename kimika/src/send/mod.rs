mod local;
mod local_grpc;
mod remote;
mod remote_grpc;
mod udp;

use clap::Args;

use crate::config;
use crossterm::{style::Stylize, tty::IsTty};

/// send file
#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct SendArgs {
    /// the path of file which want to send
    #[arg(short, long, value_name = "path")]
    pub path: Option<String>,

    /// text which wants to send
    #[arg(short, long, value_name = "message")]
    pub message: Option<String>,

    /// receiver address or remote server address. Such as: example.com
    #[arg(short, long, value_name = "address")]
    pub address: Option<String>,

    /// listen port
    #[arg(long, value_name = "port")]
    pub port: Option<u16>,

    /// receiver port when transfer from local network
    #[arg(long, value_name = "receiver_port")]
    pub receiver_port: Option<u16>,

    /// alias used for identification
    #[arg(long, value_name = "alias")]
    pub alias: Option<String>,

    /// whether to use remote server
    #[arg(short, long, value_name = "server", default_value = "false")]
    pub server: bool,
}

pub async fn send(
    args: SendArgs,
    config: &mut config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    if args.path.is_none() && args.message.is_none() && std::io::stdin().is_tty() {
        println!("{}", "Please specify a file or a message".yellow());
        return Ok(());
    }
    config.update_from_send_args(&args);

    if args.server {
        remote::remote_send(&args, &config).await?;
    } else {
        local::local_send(&args, &config).await?;
    }

    Ok(())
}
