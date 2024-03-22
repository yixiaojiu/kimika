use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

fn main() -> std::io::Result<()> {
    // 创建一个绑定到任意地址的 UDP 套接字
    let socket = UdpSocket::bind("0.0.0.0:3000")?;
    socket.set_broadcast(true)?;

    // 构建要发送的消息
    let message = "Hello, broadcast!";
    let broadcast_addr = SocketAddrV4::new(Ipv4Addr::new(255, 255, 255, 255), 3002);

    // 发送消息到广播地址
    socket.send_to(message.as_bytes(), broadcast_addr)?;

    Ok(())
}
