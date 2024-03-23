use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Entity {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct World(Vec<Entity>);

fn main() -> std::io::Result<()> {
    // 创建一个绑定到任意地址的 UDP 套接字
    let socket = UdpSocket::bind("0.0.0.0:3000")?;
    socket.set_broadcast(true)?;

    // 构建要发送的消息
    let broadcast_addr = SocketAddrV4::new(Ipv4Addr::new(255, 255, 255, 255), 3002);

    let world = World(vec![Entity { x: 0.0, y: 4.0 }, Entity { x: 10.0, y: 20.5 }]);
    let encoded: Vec<u8> = bincode::serialize(&world).unwrap();

    // 发送消息到广播地址
    socket.send_to(&encoded, broadcast_addr)?;

    Ok(())
}
