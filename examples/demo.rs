use std::net::{Ipv4Addr, SocketAddrV4};

fn main() {
    let broadcast_addr = SocketAddrV4::new(Ipv4Addr::new(255, 255, 255, 255), 230);
    println!("{}", broadcast_addr.to_string());
}
