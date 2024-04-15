use super::local_grpc::{send_file, send_message};
use super::udp::{bind_udp, broadcast, close_receiver, find_receiver};
use super::{utils, SendArgs};
use crate::config;
use crate::utils::{
    color::{print_color, Color},
    stdin_to_string,
};
use kimika_grpc::local::{local_client::LocalClient, EmptyRequest};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::oneshot::channel;
use tonic::transport::Uri;

pub async fn local_send(
    args: &SendArgs,
    config: &config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let message = utils::handle_message(args);

    let port = config.sender.as_ref().unwrap().port.unwrap();
    let receiver_port = config.sender.as_ref().unwrap().receiver_port.unwrap();

    let socket = bind_udp(port).await?;
    let socket_clone = Arc::clone(&socket);

    let address = if let Some(address) = &args.address {
        address
            .parse::<SocketAddr>()
            .expect("invalid target address")
    } else {
        let (tx, mut rx) = channel::<()>();
        print_color("searching for receiver", Color::Green);
        tokio::spawn(async move {
            broadcast(&socket_clone, receiver_port, &mut rx)
                .await
                .unwrap();
        });
        let address = find_receiver(&socket).await?;
        tx.send(()).unwrap();
        address
    };

    close_receiver(&socket, &address).await?;

    let url = format!("http://{}", address).parse::<Uri>()?;
    let mut client = LocalClient::connect(url)
        .await
        .expect("connect receiver failed");

    if args.message.is_some() {
        send_message(&mut client, message.clone().unwrap()).await;
    }

    if args.input && args.message.is_none() {
        send_message(&mut client, message.unwrap()).await;
    }

    if let Some(path) = &args.path {
        send_file(&mut client, path.clone()).await.unwrap();
    }

    client.close(EmptyRequest {}).await.unwrap();

    Ok(())
}
