use constants::SERVER_ADDRESS;

mod constants;

/* Request example
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/

fn main() {
    let server = Server::new(SERVER_ADDRESS.to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}

enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    HEAD,
    CONNECT,
    TRACE,
}

struct HttpRequest {
    method: HttpMethod,
    path: String,
    query_string: Option<String>,
}

impl HttpRequest {}
