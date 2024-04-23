use super::local_grpc::{send_file, send_message};
use super::udp::{bind_udp, broadcast, close_receiver, find_receiver};
use super::SendArgs;
use crate::{config, utils::handle, utils::select};
use kimika_grpc::local::{local_client::LocalClient, EmptyRequest};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};
use tonic::transport::Uri;

pub async fn local_send(
    args: &SendArgs,
    config: &config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let message = handle::handle_message(args);

    let receiver_port = config.sender.receiver_port;

    let socket = bind_udp(config.sender.port).await?;

    let address = if let Some(address) = &args.address {
        address
            .parse::<SocketAddr>()
            .expect("invalid target address")
    } else {
        let (tx_signal, mut rx_signal) = oneshot::channel::<()>();
        let socket_broadcast = Arc::clone(&socket);
        tokio::spawn(async move {
            broadcast(&socket_broadcast, receiver_port, &mut rx_signal)
                .await
                .unwrap();
        });

        let socket_find = Arc::clone(&socket);
        let (tx, mut rx) = mpsc::channel(1);
        tokio::spawn(async move {
            let mut receivers: Vec<select::SelectItem<String>> = Vec::new();
            loop {
                let (address, register) = find_receiver(&socket_find).await.unwrap();
                let label = format!("{} {}", register.alias, address.to_string());
                receivers.push(select::SelectItem {
                    label,
                    id: address.to_string(),
                });
                close_receiver(&socket, &address).await.unwrap();
                tx.send(receivers.clone()).await.unwrap();
            }
        });

        let address = if let Some(id) = select::receiver_select(&mut rx)
            .await
            .expect("receiver select failed")
        {
            id
        } else {
            return Ok(());
        };
        tx_signal.send(()).unwrap();
        address.parse::<SocketAddr>()?
    };

    let url = format!("http://{}", address).parse::<Uri>()?;
    let mut client = LocalClient::connect(url)
        .await
        .expect("connect receiver failed");

    if message.is_some() {
        send_message(&mut client, message.unwrap()).await;
    }

    if let Some(path) = &args.path {
        send_file(&mut client, path.clone()).await.unwrap();
    }

    client.close(EmptyRequest {}).await.unwrap();

    Ok(())
}
