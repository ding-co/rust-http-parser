use constants::SERVER_ADDRESS;

mod constants;

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
