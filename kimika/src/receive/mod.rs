pub mod grpc;
mod local;
pub mod udp;

use clap::Args;

/// receive file or message
#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct ReceiveArgs {}

pub async fn receive(_args: ReceiveArgs) -> Result<(), Box<dyn std::error::Error>> {
    local::local_receive().await?;
    Ok(())
}
