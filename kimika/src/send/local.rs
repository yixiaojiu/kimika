use super::grpc::{send_file, send_message};
use super::udp::{bind_udp, broadcast, close_receiver, find_receiver};
use super::SendArgs;
use kimika_grpc::local::{local_client::LocalClient, EmptyRequest};
use std::io::Read;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::oneshot::channel;
use tonic::transport::Uri;

pub async fn local_send(args: &SendArgs) -> Result<(), Box<dyn std::error::Error>> {
    if args.path.is_none() && args.message.is_none() && !args.input {
        eprintln!("Please specify a file or a message");
        return Ok(());
    }

    let socket = bind_udp(args.port).await?;
    let socket_clone = Arc::clone(&socket);

    let address = if let Some(target) = &args.target {
        target
            .parse::<SocketAddr>()
            .expect("invalid target address")
    } else {
        let (tx, mut rx) = channel::<()>();
        let receiver_port = args.receiver_port.clone();
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

    if let Some(message) = &args.message {
        send_message(&mut client, message.clone()).await;
    }

    if args.input && args.message.is_none() {
        let mut message = String::new();

        std::io::stdin()
            .read_to_string(&mut message)
            .expect("read standard input failed");

        send_message(&mut client, message.trim_end().to_string()).await;
    }

    if let Some(path) = &args.path {
        send_file(&mut client, path.clone()).await.unwrap();
    }

    client.close(EmptyRequest {}).await.unwrap();

    Ok(())
}
