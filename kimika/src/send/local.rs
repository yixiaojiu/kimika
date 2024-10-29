use super::SendArgs;
use crate::config;
use crate::request;
use crate::server::sender;
use crate::utils::select;

use std::net::SocketAddr;
use tokio::sync::{mpsc, oneshot};

pub async fn local_send(
    args: &SendArgs,
    config: &config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let port = config.sender.port;
    let (close_boardcast_tx, close_boardcast_rx) = oneshot::channel::<()>();

    let broadcast_addr: SocketAddr = ([255, 255, 255, 255], port).into();

    tokio::spawn(async move {
        request::udp::broadcast(broadcast_addr, port, close_boardcast_rx)
            .await
            .unwrap();
    });

    let (receiver_tx, mut receiver_rx) = mpsc::channel(1);
    let (close_server_tx, close_server_rx) = oneshot::channel::<()>();
    tokio::spawn(async move {
        sender::start_server(port, receiver_tx, close_server_rx)
            .await
            .unwrap();
    });

    let (options_tx, mut options_rx) = mpsc::channel(1);
    tokio::spawn(async move {
        let mut options: Vec<select::SelectItem<String>> = Vec::new();

        loop {
            match receiver_rx.recv().await {
                Some(receiver) => {
                    let address = receiver.address.to_string();
                    if !options.iter().any(|option| option.id == address) {
                        continue;
                    }
                    options.push(select::SelectItem {
                        id: address,
                        label: format!("{:12} {}", receiver.alias, receiver.address),
                    });
                    options_tx.send(options.clone()).await.unwrap();
                    continue;
                }
                None => break,
            }
        }
    });

    let address: SocketAddr = if let Some(selected_item) = select::receiver_select(&mut options_rx)
        .await
        .expect("select receiver failed")
    {
        selected_item.id.parse().unwrap()
    } else {
        return Ok(());
    };
    // close channel
    drop(options_rx);

    Ok(())
}
