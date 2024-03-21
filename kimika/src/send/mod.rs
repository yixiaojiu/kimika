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

pub fn send(args: SendArgs) {
    if let Some(path) = args.path {
        println!("send file: {}", path);
    }

    if let Some(message) = args.message {
        println!("send message: {}", message);
    }
}
