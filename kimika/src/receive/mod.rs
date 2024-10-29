mod local;
mod remote;
mod udp;

use crate::config;
use clap::Args;

/// Receive file or message
#[derive(Args, Debug)]
#[command(version, about, alias = "recv", long_about = None)]
pub struct ReceiveArgs {
    /// Listen port
    #[arg(long, value_name = "port")]
    pub port: Option<u16>,

    /// Save folder
    #[arg(short, long, value_name = "folder")]
    pub folder: Option<String>,

    /// Alias used for identification
    #[arg(long, value_name = "alias")]
    pub alias: Option<String>,

    /// Whether to use remote server
    #[arg(short, long, value_name = "server", default_value = "false")]
    pub server: bool,

    /// Remote server address. Such as: example.com
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
