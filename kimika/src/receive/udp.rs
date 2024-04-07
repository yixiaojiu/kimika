use crate::utils::udp::BUFFER_SIZE;
use crate::utils::udp::{Action, Register};
use bincode::{deserialize, serialize};
use std::net::SocketAddrV4;
use tokio::net::UdpSocket;

pub async fn listen_boardcast(
    address: &SocketAddrV4,
    alias: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind(address).await?;
    socket.set_broadcast(true)?;
    let mut buffer = [0u8; BUFFER_SIZE];

    loop {
        let (num_bytes, address) = socket.recv_from(&mut buffer).await?;
        let action: Action = deserialize(&buffer[..num_bytes])?;

        match action {
            Action::Broadcast => {
                let message = Register {
                    alias: alias.clone(),
                };
                let buf = serialize(&message)?;
                socket.send_to(&buf, &address).await?;
            }
            Action::Close => {
                break;
            }
        };
    }
    Ok(())
}
