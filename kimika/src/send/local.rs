use super::grpc::{send_file, send_message};
use super::udp::{bind_udp, broadcast, close_receiver, find_receiver};
use super::SendArgs;
use crate::utils::{
    color::{print_color, Color},
    stdin_to_string,
};
use kimika_grpc::local::{local_client::LocalClient, EmptyRequest};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::oneshot::channel;
use tonic::transport::Uri;

pub async fn local_send(args: &SendArgs) -> Result<(), Box<dyn std::error::Error>> {
    if args.path.is_none() && args.message.is_none() && !args.input {
        print_color("Please specify a file or a message", Color::Yellow);
        return Ok(());
    }

    let message = if let Some(message) = &args.message {
        message.clone()
    } else if args.input {
        stdin_to_string().trim_end().to_string()
    } else {
        String::new()
    };

    let socket = bind_udp(args.port).await?;
    let socket_clone = Arc::clone(&socket);

    let address = if let Some(target) = &args.target {
        target
            .parse::<SocketAddr>()
            .expect("invalid target address")
    } else {
        let (tx, mut rx) = channel::<()>();
        let receiver_port = args.receiver_port.clone();
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
        send_message(&mut client, message.clone()).await;
    }

    if args.input && args.message.is_none() {
        send_message(&mut client, message).await;
    }

    if let Some(path) = &args.path {
        send_file(&mut client, path.clone()).await.unwrap();
    }

    client.close(EmptyRequest {}).await.unwrap();

    Ok(())
}
