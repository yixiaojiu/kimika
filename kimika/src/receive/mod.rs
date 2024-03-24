mod grpc;
mod local;
mod udp;

use clap::Args;

/// receive file or message
#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct ReceiveArgs {
    // TODO: move port to config
    #[arg(long, default_value = "3939")]
    pub port: u16,
}

pub async fn receive(args: ReceiveArgs) -> Result<(), Box<dyn std::error::Error>> {
    local::local_receive(args).await?;
    Ok(())
}
