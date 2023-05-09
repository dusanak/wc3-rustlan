use std::io::ErrorKind::{BrokenPipe, ConnectionAborted, TimedOut, WouldBlock};
use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

pub struct TcpProxy {
    client: TcpStream,
    server: TcpStream,
}

impl TcpProxy {
    pub fn new(address: String, client: TcpStream) -> Self {
        let server = TcpStream::connect(&address).unwrap();
        println!("Connected to remote server {}.", address); 

        // client.set_nonblocking(true).unwrap();
        // server.set_nonblocking(true).unwrap();

        client
            .set_read_timeout(Some(Duration::from_millis(100)))
            .unwrap();
        server
            .set_read_timeout(Some(Duration::from_millis(100)))
            .unwrap();

        Self { client, server }
    }

    pub fn start(&mut self) {
        let mut buf = [0; 256];

        loop {
            match self.client.read(&mut buf) {
                Ok(received) => match self.server.write(&buf[..received]) {
                    Ok(_) => {}
                    Err(ref e) if e.kind() == BrokenPipe => break,
                    Err(e) => panic!("encountered IO error: {e}"),
                },
                Err(ref e) if (e.kind() == WouldBlock) | (e.kind() == TimedOut) => {}
                Err(ref e) if e.kind() == ConnectionAborted => break,
                Err(e) => panic!("encountered IO error: {e}"),
            }

            match self.server.read(&mut buf) {
                Ok(received) => match self.client.write(&buf[..received]) {
                    Ok(_) => {}
                    Err(ref e) if e.kind() == BrokenPipe => break,
                    Err(e) => panic!("encountered IO error: {e}"),
                },
                Err(ref e) if (e.kind() == WouldBlock) | (e.kind() == TimedOut) => {}
                Err(ref e) if e.kind() == ConnectionAborted => break,
                Err(e) => panic!("encountered IO error: {e}"),
            }
        }

        println!(
            "Connection to {} terminated.",
            self.server.peer_addr().unwrap()
        );
    }
}
