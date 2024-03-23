use crate::utils::color::paint_green;
use crate::utils::udp::{Action, Register, BUFFER_SIZE};
use bincode::{deserialize, serialize};
use std::net::SocketAddrV4;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::sync::oneshot::{Receiver, Sender};
use tokio::time::sleep;

const SLEEP_DURATION: Duration = Duration::from_millis(300);

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
    tx: Sender<()>,
) -> Result<SocketAddr, Box<dyn std::error::Error>> {
    let mut buffer = [0u8; BUFFER_SIZE];
    let (num_bytes, address) = socket.recv_from(&mut buffer).await?;

    let register: Register = deserialize(&buffer[..num_bytes])?;
    println!(
        "Find a receiver: {} {}",
        paint_green(&address.to_string()),
        paint_green(&register.alias)
    );

    // let buf = serialize(&Action::Close).unwrap();
    // socket.send_to(&buf, address).await?;
    // close boardcast
    let _ = tx.send(());
    Ok(address)
}
