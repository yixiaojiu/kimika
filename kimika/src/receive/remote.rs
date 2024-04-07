use super::{remote_grpc, ReceiveArgs};
use crate::utils::color::{print_color, Color};
use kimika_grpc::remote;
use std::net::SocketAddr;

pub async fn remote_receive(args: &ReceiveArgs) -> Result<(), Box<dyn std::error::Error>> {
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

    let register_res = remote_grpc::register_receiver(&mut client, &args.alias)
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

    remote_grpc::receive(&mut client, &receiver_id, &args.save_folder, content)
        .await
        .expect("receive failed");

    Ok(())
}
