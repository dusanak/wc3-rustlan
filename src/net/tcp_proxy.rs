use std::{net::TcpStream, io::{Read, self, Write}};

pub struct TcpProxy {
    client: TcpStream,
    server: TcpStream,
}

impl TcpProxy {
    pub fn new(address: String, client: TcpStream) -> Self {
        let server = TcpStream::connect(address).unwrap();

        server.set_nonblocking(true).unwrap();
        client.set_nonblocking(true).unwrap();

        Self {
            client,
            server,
        }
    }

    pub fn start(&mut self) {
        let mut buf = [0; 256]; 

        loop {
            match self.client.read(&mut buf) {
                Ok(received) => {
                    self.server.write(&buf[..received]).unwrap();
                },
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {},
                Err(ref e) if e.kind() == io::ErrorKind::ConnectionAborted => break,
                Err(e) => panic!("encountered IO error: {e}"),
            }

            match self.server.read(&mut buf) {
                Ok(received) => {
                    self.client.write(&buf[..received]).unwrap();
                },
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {},
                Err(ref e) if e.kind() == io::ErrorKind::ConnectionAborted => break,
                Err(e) => panic!("encountered IO error: {e}"),
            }
        }

        println!("Connection with {} aborted.", self.server.peer_addr().unwrap());
    }
}