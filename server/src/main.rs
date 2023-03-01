#![allow(dead_code)]

use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

/* 간단한 HTTP 요청 가이드
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path)); // 어떤 TCP 소켓에 생긴 새로운 연결을 기다리면서 영원히 반복
}
