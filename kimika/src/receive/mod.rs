mod grpc;
mod local;
mod udp;

use clap::Args;

/// receive file or message
#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct ReceiveArgs {
    #[arg(long, default_value = "3939", value_name = "port")]
    pub port: u16,

    #[arg(long, default_value = "", value_name = "save_folder")]
    pub save_folder: String,

    /// receiver alias
    #[arg(long, default_value = "bar", value_name = "alias")]
    pub alias: String,
}

pub async fn receive(args: ReceiveArgs) -> Result<(), Box<dyn std::error::Error>> {
    local::local_receive(args).await?;
    Ok(())
}
