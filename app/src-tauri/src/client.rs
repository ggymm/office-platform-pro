use std::net::{Ipv4Addr, TcpStream};

pub fn init_tcp_client() {
    println!("初始化 Tcp Client 服务");

    let _stream = TcpStream::connect(
        (Ipv4Addr::new(127, 0, 0, 1), 20000)
    );
}