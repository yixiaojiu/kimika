use super::{remote_grpc, ReceiveArgs};
use crate::{config, utils::handle};
use crossterm::style::Stylize;
use kimika_grpc::remote;

pub async fn remote_receive(
    args: &ReceiveArgs,
    config: &config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let address = if let Some(addr) = handle::handle_address(args.address.clone(), config) {
        addr
    } else {
        println!("{}", "No server address configured".red());
        return Ok(());
    };

    let alias = config.alias.clone();
    let save_folder = std::path::PathBuf::from(config.receiver.save_folder.clone());

    let mut client = remote_grpc::create_client(address)
        .await
        .expect("connect remote server failed");
    eprintln!("{} {}", "Connected to remote server: ".green(), address);

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
