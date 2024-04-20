use super::local_grpc::{send_file, send_message};
use super::udp::{bind_udp, broadcast, close_receiver, find_receiver};
use super::{utils, SendArgs};
use crate::{config, utils::select::Select};
use crossterm::{cursor, execute, style::Stylize, terminal};
use kimika_grpc::local::{local_client::LocalClient, EmptyRequest};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};
use tonic::transport::Uri;

pub async fn local_send(
    args: &SendArgs,
    config: &config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let message = utils::handle_message(args);

    let port = config.sender.as_ref().unwrap().port.unwrap();
    let receiver_port = config.sender.as_ref().unwrap().receiver_port.unwrap();

    let socket = bind_udp(port).await?;

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
            let mut receivers: Vec<String> = Vec::new();
            loop {
                let (address, register) = find_receiver(&socket_find).await.unwrap();
                receivers.push(format!("{} {}", register.alias, address.to_string()));
                close_receiver(&socket, &address).await.unwrap();
                tx.send(receivers.clone()).await.unwrap();
            }
        });
        println!("Select a receiver >> (Press q to exit)");
        let mut select = Select::new(Vec::new(), std::io::stdout());
        let receiver_str = select.start(&mut rx).await;
        if receiver_str.is_none() {
            return Ok(());
        }
        let receiver_str = receiver_str.unwrap();

        tx_signal.send(()).unwrap();
        let receiver_vec = receiver_str.split(' ').collect::<Vec<&str>>();
        execute!(
            std::io::stdout(),
            cursor::MoveToPreviousLine(1u16),
            terminal::Clear(terminal::ClearType::FromCursorDown),
        )
        .unwrap();
        println!(
            "Select a receiver >> {} {}",
            receiver_vec[0].cyan(),
            receiver_vec[1].cyan()
        );

        receiver_vec[1].parse::<SocketAddr>()?
    };

    let url = format!("http://{}", address).parse::<Uri>()?;
    let mut client = LocalClient::connect(url)
        .await
        .expect("connect receiver failed");

    if args.message.is_some() || args.input {
        send_message(&mut client, message.unwrap()).await;
    }

    if let Some(path) = &args.path {
        send_file(&mut client, path.clone()).await.unwrap();
    }

    client.close(EmptyRequest {}).await.unwrap();

    Ok(())
}
