use std::net::UdpSocket;

const BUFFER_SIZE: usize = 1024;

fn main() -> std::io::Result<()> {
    // 创建一个绑定到广播地址的 UDP 套接字
    let socket = UdpSocket::bind("0.0.0.0:3002")?;
    socket.set_broadcast(true)?;

    let mut buffer = [0u8; BUFFER_SIZE];

    loop {
        // 接收广播消息
        let (num_bytes, src_addr) = socket.recv_from(&mut buffer)?;
        let message = std::str::from_utf8(&buffer[..num_bytes]).unwrap();

        println!("Received broadcast message: {}", message);
        println!("From: {}", src_addr);
    }
}
