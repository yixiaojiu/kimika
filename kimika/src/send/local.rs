use super::SendArgs;
use crate::utils::udp::{Action, Register};
use bincode::{deserialize, serialize};
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::net::UdpSocket;

const BUFFER_SIZE: usize = 1024;
const CLIENT_PROT: u16 = 3002;

pub async fn local_send(_args: &SendArgs) -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:3000").await?;
    socket.set_broadcast(true)?;
    let broadcast_addr = SocketAddrV4::new(Ipv4Addr::new(255, 255, 255, 255), CLIENT_PROT);

    let buf = serialize(&Action::Broadcast).unwrap();
    socket.send_to(&buf, broadcast_addr).await?;

    let mut buffer = [0u8; BUFFER_SIZE];
    let (num_bytes, address) = socket.recv_from(&mut buffer).await?;

    let register: Register = deserialize(&buffer[..num_bytes])?;
    println!("Received register alias: {}", register.alias);
    println!("From: {}", address);

    let buf = serialize(&Action::Close).unwrap();
    socket.send_to(&buf, address).await?;

    Ok(())
}
