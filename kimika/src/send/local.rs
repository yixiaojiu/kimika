use super::udp::{bind_udp, broadcast, find_receiver};
use super::SendArgs;
use crate::transfer::{transfer_client::TransferClient, MessageRequest};
use std::sync::Arc;
use tokio::sync::oneshot::channel;
use tonic::transport::Uri;

pub async fn local_send(args: &SendArgs) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: move port to config
    let port: u16 = 3000;
    let socket = bind_udp(port).await?;

    let socket_clone = Arc::clone(&socket);
    let (tx, mut rx) = channel::<()>();
    tokio::spawn(async move {
        // TODO: move port to config
        let port: u16 = 3002;
        broadcast(&socket_clone, port, &mut rx).await.unwrap();
    });

    let address = find_receiver(&socket, tx).await?;

    let url = format!("http://{}", address).parse::<Uri>()?;
    let mut client = TransferClient::connect(url).await?;

    client
        .send_message(MessageRequest {
            message: args.message.clone().unwrap(),
        })
        .await
        .unwrap();

    Ok(())
}
