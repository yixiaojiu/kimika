mod local;
mod local_grpc;
mod remote;
mod remote_grpc;
mod udp;
mod utils;

use clap::Args;

use crate::config;
use crate::utils::color::{print_color, Color};

/// send file
#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct SendArgs {
    #[arg(short, long, value_name = "path")]
    pub path: Option<String>,

    #[arg(short, long, value_name = "message")]
    pub message: Option<String>,

    /// receiver address or remote server address. Such as: example.com
    #[arg(short, long, value_name = "address")]
    pub address: Option<String>,

    #[arg(long, value_name = "port")]
    pub port: Option<u16>,

    /// receiver port when transfer from local network
    #[arg(long, value_name = "receiver_port")]
    pub receiver_port: Option<u16>,

    /// whether to read message from standard input, press ctrl_d to end input
    #[arg(short, long, value_name = "input")]
    pub input: bool,

    /// whether to use remote server
    #[arg(short, long, value_name = "server", default_value = "false")]
    pub server: bool,
}

pub async fn send(
    args: SendArgs,
    config: &mut config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    if args.path.is_none() && args.message.is_none() && !args.input {
        print_color("Please specify a file or a message", Color::Yellow);
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
