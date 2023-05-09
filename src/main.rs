mod net;
mod warcraft;

use std::{env, net::TcpListener, thread::spawn, sync::mpsc};

use net::info_client::InfoClient;

use crate::net::tcp_proxy::TcpProxy;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Not enough parameters passed.");
        return;
    }

    println!("WC3 RustLAN started.");

    let listener = TcpListener::bind("0.0.0.0:0").unwrap();

    println!("Remote server address is {}.", args[1]);

    let (tx, rx) = mpsc::channel();

    let addr = args[1].clone();
    let port = listener.local_addr().unwrap().port();
    let info_client_handle = spawn(move || {
        let mut info_client = InfoClient::new(port);
        info_client.start(addr, rx);
    });

    let (client, sock) = listener.accept().unwrap();
    println!("Accepted connection from {}", sock);
    _ = tx.send(());

    let addr = args[1].clone();
    let tcp_proxy_handle = spawn(move || {
        let mut tcp_proxy = TcpProxy::new(addr, client);
        tcp_proxy.start();
    });

    info_client_handle.join().unwrap();
    tcp_proxy_handle.join().unwrap();
}
