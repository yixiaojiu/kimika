mod grpc;
mod local;
mod udp;

use clap::Args;

/// send file
#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct SendArgs {
    #[arg(short, long, value_name = "path")]
    pub path: Option<String>,

    #[arg(short, long, value_name = "message")]
    pub message: Option<String>,

    #[arg(short, long, value_name = "target")]
    pub target: Option<String>,

    #[arg(long, default_value = "3939", value_name = "port")]
    pub port: u16,

    #[arg(long, default_value = "3939", value_name = "receiver_port")]
    pub receiver_port: u16,

    /// whether to read message from standard input, press ctrl_d to end input
    #[arg(short, long, value_name = "input")]
    pub input: bool,
}

pub async fn send(args: SendArgs) -> Result<(), Box<dyn std::error::Error>> {
    local::local_send(&args).await?;
    Ok(())
}
