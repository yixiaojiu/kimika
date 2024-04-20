use crate::utils::udp::{Action, Register, BUFFER_SIZE};
use bincode::{deserialize, serialize};
use std::net::SocketAddrV4;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::sync::oneshot::Receiver;
use tokio::time::sleep;

const SLEEP_DURATION: Duration = Duration::from_millis(100);

/// create udp listener
pub async fn bind_udp(port: u16) -> Result<Arc<UdpSocket>, Box<dyn std::error::Error>> {
    let address = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);
    let socket = Arc::new(UdpSocket::bind(address).await?);
    socket.set_broadcast(true)?;
    Ok(socket)
}

pub async fn broadcast(
    socket: &UdpSocket,
    port: u16,
    rx: &mut Receiver<()>,
) -> Result<(), Box<dyn std::error::Error>> {
    let broadcast_addr = SocketAddrV4::new(Ipv4Addr::new(255, 255, 255, 255), port);
    let buf = serialize(&Action::Broadcast).unwrap();
    loop {
        if let Ok(_) = rx.try_recv() {
            break;
        }
        socket.send_to(&buf, broadcast_addr).await?;
        sleep(SLEEP_DURATION).await;
    }

    Ok(())
}

pub async fn find_receiver(
    socket: &UdpSocket,
) -> Result<(SocketAddr, Register), Box<dyn std::error::Error>> {
    let mut buffer = [0u8; BUFFER_SIZE];
    let (num_bytes, address) = socket.recv_from(&mut buffer).await?;

    let register: Register = deserialize(&buffer[..num_bytes])?;
    Ok((address, register))
}

pub async fn close_receiver(
    socket: &UdpSocket,
    address: &SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    let buf = serialize(&Action::Close)?;
    socket.send_to(&buf, address).await?;
    Ok(())
}
