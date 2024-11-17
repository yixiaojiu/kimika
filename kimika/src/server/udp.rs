use crate::request::local as request_local;
use crate::CONFIG;

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio::sync::oneshot;

const BUFFER_SIZE: usize = 1024;

#[derive(Deserialize, Serialize, Debug)]
pub struct UDPPacket {
    /// sender linsten port
    pub port: u16,
}

pub async fn listen_boardcast(mut close_rx: oneshot::Receiver<()>) -> Result<(), std::io::Error> {
    let address: SocketAddr = ([0, 0, 0, 0], CONFIG.sender.receiver_port).into();
    let socket = UdpSocket::bind(address).await?;
    socket.set_broadcast(true)?;

    socket.join_multicast_v4("224.0.0.139".parse().unwrap(), "0.0.0.0".parse().unwrap())?;
    let mut buffer = vec![0u8; BUFFER_SIZE];

    loop {
        tokio::select! {
            _ = &mut close_rx => {
                break;
            }
            socket_recv = socket.recv_from(&mut buffer) => {
                let (num_bytes, address) = socket_recv?;

                if let Ok(packet) = serde_json::from_slice::<UDPPacket>(&buffer[..num_bytes]) {
                    let request =
                        request_local::RequestClient::new(&SocketAddr::new(address.ip(), packet.port));
                    request.register(CONFIG.alias.clone(), CONFIG.receiver.port).await.unwrap();
                };
                continue;
            }
        }
    }

    Ok(())
}
