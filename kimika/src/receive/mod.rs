mod local;
mod local_grpc;
mod remote;
mod remote_bark;
mod remote_grpc;
mod udp;

use crate::config;
use clap::Args;

/// receive file or message
#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct ReceiveArgs {
    /// listen port
    #[arg(long, value_name = "port")]
    pub port: Option<u16>,

    /// save folder
    #[arg(short, long, value_name = "folder")]
    pub folder: Option<String>,

    /// alias used for identification
    #[arg(long, value_name = "alias")]
    pub alias: Option<String>,

    /// whether to use remote server
    #[arg(short, long, value_name = "server", default_value = "false")]
    pub server: bool,

    /// remote server address. Such as: example.com
    #[arg(short, long, value_name = "address")]
    pub address: Option<String>,
}

pub async fn receive(
    args: ReceiveArgs,
    config: &mut config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    config.update_from_receive_args(&args);

    if args.server {
        remote::remote_receive(&args, &config).await?;
    } else {
        local::local_receive(&args, &config).await?;
    }

    Ok(())
}
