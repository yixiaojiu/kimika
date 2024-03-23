use crate::utils::udp::{Action, Register};
use bincode::{deserialize, serialize};
use tokio::net::UdpSocket;

const BUFFER_SIZE: usize = 1024;

pub async fn udp_handle() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:3002").await?;
    socket.set_broadcast(true)?;
    let mut buffer = [0u8; BUFFER_SIZE];

    // TODO: 放到配置里
    let alias = "bar";
    loop {
        let (num_bytes, address) = socket.recv_from(&mut buffer).await?;
        let action: Action = deserialize(&buffer[..num_bytes])?;

        match action {
            Action::Broadcast => {
                let message = Register {
                    alias: alias.to_string(),
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
