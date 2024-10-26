use serde::Deserialize;
use std::net::SocketAddrV4;
use tokio::net::UdpSocket;
use tokio::sync::oneshot;

const BUFFER_SIZE: usize = 1024;

#[derive(Deserialize)]
struct UDPPacket {}

pub async fn listen_boardcast(
    address: &SocketAddrV4,
    mut close_rx: oneshot::Receiver<()>,
) -> Result<(), std::io::Error> {
    let socket = UdpSocket::bind(address).await?;
    socket.set_broadcast(true)?;
    let mut buffer = vec![0u8; BUFFER_SIZE];

    loop {
        if close_rx.try_recv().is_ok() {
            break;
        }
        let (num_bytes, _address) = socket.recv_from(&mut buffer).await?;
        let _packet: UDPPacket = serde_json::from_slice(&buffer[..num_bytes])?;
    }

    Ok(())
}
