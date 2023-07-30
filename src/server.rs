use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { address: addr }
    }

    pub fn run(self) {
        println!("Starting up...");
        println!("Listening on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(Request) => {}
                                Err(e) => println!("Failed to from a request: {e}"),
                            }
                        }
                        Err(e) => println!("Failed to read buffer: {e}"),
                    }
                }
                Err(e) => println!("Failed to accept connection: {e}"),
            }
        }
    }
}
