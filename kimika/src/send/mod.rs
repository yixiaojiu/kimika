mod grpc;
mod local;
mod udp;

use clap::Args;

/// send file
#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct SendArgs {
    #[arg(short, long)]
    pub path: Option<String>,

    #[arg(short, long)]
    pub message: Option<String>,

    #[arg(short, long)]
    pub target: Option<String>,

    // TODO: move port to config
    #[arg(long, default_value = "3939")]
    pub port: u16,

    // TODO: move port to config
    #[arg(long, default_value = "3939")]
    pub receiver_port: u16,
}

pub async fn send(args: SendArgs) -> Result<(), Box<dyn std::error::Error>> {
    local::local_send(&args).await?;
    Ok(())
}
