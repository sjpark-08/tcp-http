use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // 서버 초기화 후 IP와 포트 3000에 바인딩
    let connection_listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000");
    // 유입되는 커넥션을 기다림 (listen)
    for stream in connection_listener.incoming() {
        let mut stream = stream.unwrap(); // 커넥션 유입
        println!("Connection established!");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
