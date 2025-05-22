use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server <'a> {
    socket_addr: &'a str,
}
impl<'a> Server <'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }
    
    pub fn run(&self) {
        // 소켓 주소 리스닝하는 서버 시작
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);
        // 루프 안에서 유입되는 커넥션을 리스닝한다.
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");
            let mut read_buffer = [0; 90];
            stream.read(&mut read_buffer).unwrap();
            // HTTP 요청을 러스트 데이터 구조로 변환
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            // 요청을 적절한 핸들러로 라우팅한다
            Router::route(req, &mut stream);
        }
    }
}