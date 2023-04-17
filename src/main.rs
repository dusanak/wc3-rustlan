mod net;
mod warcraft;

use std::{env, net::TcpListener, thread::spawn};

use net::info_client::InfoClient;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Not enough parameters passed.");
        return;
    }

    let listener = TcpListener::bind("0.0.0.0:0").unwrap();
    
    let port = listener.local_addr().unwrap().port();
    let info_client_handle = spawn(move || {
        let mut info_client = InfoClient::new(port);
        info_client.start(&args[1]);
    });

    let (_stream, sock) = listener.accept().unwrap();
    println!("Accepted connection from {}", sock);

    info_client_handle.join().unwrap();
}
