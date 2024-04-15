use super::{remote_grpc, ReceiveArgs};
use crate::config;
use kimika_grpc::remote;
use std::net::SocketAddr;

pub async fn remote_receive(
    _args: &ReceiveArgs,
    config: &config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let address = config
        .server
        .as_ref()
        .unwrap()
        .address
        .clone()
        .unwrap()
        .parse::<SocketAddr>()
        .expect("invalid target address");
    let receiver = config.receiver.as_ref().unwrap();
    let alias = receiver.alias.clone().unwrap();
    let save_folder = std::path::PathBuf::from(receiver.save_folder.clone().unwrap());

    let mut client = remote_grpc::create_client(address)
        .await
        .expect("connect remote server failed");

    let register_res = remote_grpc::register_receiver(&mut client, &alias)
        .await
        .expect("register receiver failed");

    let receiver_id = register_res.receiver_id;

    let mut content_res = remote_grpc::get_content(&mut client, &receiver_id)
        .await
        .expect("get content failed");

    let mut content = remote::get_content_response::Content::default();
    while let Some(res) = content_res.message().await? {
        for item in res.content_list {
            content = item;
        }
        break;
    }

    remote_grpc::receive(&mut client, &receiver_id, &save_folder, content)
        .await
        .expect("receive failed");

    Ok(())
}
