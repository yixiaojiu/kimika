use super::{remote_grpc, utils, SendArgs};
use crate::utils::color::{print_color, Color};
use std::{net::SocketAddr, path::PathBuf};

#[warn(dead_code)]
pub struct Content {
    pub message: Option<String>,
    pub path: Option<PathBuf>,
    pub name: Option<String>,
    pub size: Option<u64>,
}

pub async fn remote_send(args: &SendArgs) -> Result<(), Box<dyn std::error::Error>> {
    let message = utils::handle_message(args);

    let content = if let Some(path) = &args.path {
        Content {
            message: None,
            path: Some(PathBuf::from(path)),
            name: None,
            size: None,
        }
    } else {
        Content {
            message,
            path: None,
            name: None,
            size: None,
        }
    };

    let address = if let Some(address) = &args.address {
        address
            .parse::<SocketAddr>()
            .expect("invalid target address")
    } else {
        print_color("please input remote server address", Color::Red);
        return Ok(());
    };

    let mut client = remote_grpc::create_client(address)
        .await
        .expect("connect remote server failed");

    let register_res = remote_grpc::register_content(&mut client, &content)
        .await
        .expect("register content failed");

    Ok(())
}
