use clap::Args;

/// send file
#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct ReceiveArgs {}

pub fn receive(_args: ReceiveArgs) {}
