mod local;
mod local_grpc;
mod remote;
mod remote_grpc;
mod udp;
use std::path::PathBuf;

use clap::Args;

/// receive file or message
#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct ReceiveArgs {
    #[arg(long, default_value = "3939", value_name = "port")]
    pub port: u16,

    #[arg(long, default_value = "./", value_name = "save_folder")]
    pub save_folder: PathBuf,

    /// receiver alias
    #[arg(long, default_value = "receiver", value_name = "alias")]
    pub alias: String,

    /// whether to use remote server
    #[arg(short, long, value_name = "server", default_value = "false")]
    pub server: bool,

    /// remote server address. Such as: example.com
    #[arg(short, long, value_name = "address")]
    pub address: Option<String>,
}

pub async fn receive(args: ReceiveArgs) -> Result<(), Box<dyn std::error::Error>> {
    if args.server {
        remote::remote_receive(&args).await?;
    } else {
        local::local_receive(&args).await?;
    }
    Ok(())
}
