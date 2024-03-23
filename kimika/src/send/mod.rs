mod local;

use clap::Args;

/// send file
#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct SendArgs {
    #[arg(short, long)]
    pub path: Option<String>,

    #[arg(short, long)]
    pub message: Option<String>,
}

pub async fn send(args: SendArgs) -> Result<(), Box<dyn std::error::Error>> {
    local::local_send(&args).await?;
    Ok(())
}
