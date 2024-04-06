mod local;
mod local_grpc;
mod remote;
mod remote_grpc;
mod udp;
mod utils;

use clap::Args;

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
    #[arg(short, long, value_name = "target")]
    pub address: Option<String>,

    #[arg(long, default_value = "3939", value_name = "port")]
    pub port: u16,

    /// receiver port when transfer from local network
    #[arg(long, default_value = "3939", value_name = "receiver_port")]
    pub receiver_port: u16,

    /// whether to read message from standard input, press ctrl_d to end input
    #[arg(short, long, value_name = "input")]
    pub input: bool,

    /// whether to use remote server
    #[arg(short, long, value_name = "server", default_value = "false")]
    pub server: bool,
}

pub async fn send(args: SendArgs) -> Result<(), Box<dyn std::error::Error>> {
    if args.path.is_none() && args.message.is_none() && !args.input {
        print_color("Please specify a file or a message", Color::Yellow);
        return Ok(());
    }

    if args.server {
        remote::remote_send(&args).await?;
    } else {
        local::local_send(&args).await?;
    }

    Ok(())
}
