use constants::SERVER_ADDRESS;
use server::Server;

mod constants;
mod http;
mod server;

/* Request example
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/

fn main() {
    let server = Server::new(SERVER_ADDRESS.to_string());
    server.run();
}
