use super::{remote_grpc, SendArgs};
use crate::{config, utils::handle, utils::select};
use crossterm::style::Stylize;
use std::{fs, path::PathBuf};
use tokio::sync::mpsc;

pub struct Content {
    pub message: Option<String>,
    pub path: Option<PathBuf>,
    pub name: Option<String>,
    pub size: Option<u64>,
}

pub async fn remote_send(
    args: &SendArgs,
    config: &config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let message = handle::handle_message(args);

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

    let address = if let Some(addr) = handle::handle_address(args.address.clone(), config) {
        addr
    } else {
        println!("{}", "No server address configured".red());
        return Ok(());
    };

    let mut client = remote_grpc::create_client(address)
        .await
        .expect("connect remote server failed");
    println!("{} {}", "Connected to remote server: ".green(), address);

    let register_res = remote_grpc::register_content(&mut client, &content)
        .await
        .expect("register content failed");

    let mut receiver_res = remote_grpc::get_receivers(&mut client)
        .await
        .expect("get receivers failed");

    #[allow(unused_assignments)]
    let mut receiver_id = String::new();
    let sender_id = register_res.sender_id;
    let content_id = register_res.content_id;

    let (tx, mut rx) = mpsc::channel(1);
    tokio::spawn(async move {
        while let Some(res) = receiver_res.message().await.unwrap() {
            let receiver_iter = res.receivers.iter().map(|receiver| select::SelectItem {
                label: format!("{} {}", receiver.alias, receiver.ip),
                id: receiver.receiver_id.clone(),
            });
            tx.send(receiver_iter.collect()).await.unwrap();
        }
    });

    if let Some(id) = select::receiver_select(&mut rx)
        .await
        .expect("select receiver failed")
    {
        receiver_id = id;
    } else {
        return Ok(());
    }

    let mut choose_res =
        remote_grpc::choose_receiver(&mut client, receiver_id.clone(), sender_id.clone())
            .await
            .expect("request choose receiver failed");

    while let Some(_) = choose_res.message().await? {
        println!("Start sending");
        break;
    }

    remote_grpc::send(&mut client, content_id, &content)
        .await
        .expect("send content failed");

    Ok(())
}
