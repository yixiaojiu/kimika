use super::{remote_grpc, utils, SendArgs};
use crate::utils::color::{self, print_color, Color};
use std::{fs, net::SocketAddr, path::PathBuf};

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
        let pathbuf = PathBuf::from(path);
        if !pathbuf.exists() {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "file not exists",
            ))?
        }
        let metadata = fs::metadata(&pathbuf).expect("get metadata failed");
        let filename = pathbuf
            .file_name()
            .expect("invalid file name")
            .to_str()
            .unwrap();
        Content {
            message: None,
            path: Some(pathbuf.clone()),
            name: Some(filename.to_string()),
            size: Some(metadata.len()),
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

    let mut receiver_res = remote_grpc::get_receivers(&mut client)
        .await
        .expect("get receivers failed");

    let mut receiver_id = String::new();
    let sender_id = register_res.sender_id;
    let content_id = register_res.content_id;

    while let Some(res) = receiver_res.message().await? {
        let receiver_iter = res.receivers.iter().map(|receiver| {
            receiver_id = receiver.receiver_id.clone();
            color::paint_green(format!("{} {}", receiver.ip, receiver.alias))
        });

        for receiver in receiver_iter {
            println!("{}", receiver);
        }
        break;
    }

    let mut choose_res =
        remote_grpc::choose_receiver(&mut client, receiver_id.clone(), sender_id.clone())
            .await
            .expect("request choose receiver failed");

    while let Some(res) = choose_res.message().await? {
        println!("start sending, receiver_id: {}", res.receiver_id);
        break;
    }

    remote_grpc::send(&mut client, content_id, &content)
        .await
        .expect("send content failed");

    Ok(())
}
