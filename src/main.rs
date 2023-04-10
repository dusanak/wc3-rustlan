mod warcraft {
    pub mod expansion;
    pub mod game_info;
    pub mod query_protocol;
}

use std::{
    env,
    net::{SocketAddrV4, UdpSocket},
};

use warcraft::{expansion::Expansion, query_protocol::{extract_game_info, get_browse_packet}};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Not enough parameters passed.");
        return;
    }

    let socket = UdpSocket::bind("0.0.0.0:6112").expect("Failed to bind UDP socket.");

    let addr: SocketAddrV4 = args[1].parse().unwrap_or_else(|_| -> SocketAddrV4 {
        println!("Defaulting to port 6112.");
        SocketAddrV4::new(args[1].parse().expect("Failed to parse IP address"), 6112)
    });
    socket.connect(addr).expect("Connect function failed.");

    println!("Sending a browse packet to {}.", addr);
    socket
        .send(&get_browse_packet(Expansion::ROC, 26))
        .expect(&format!("Failed to send browse packet to {}", addr));

    println!("Waiting for game info.");
    let mut buf = [0; 256];
    let received = socket.recv(&mut buf).expect("Failed to receive game info.");

    let game_info = extract_game_info(&buf[..received]).expect("Failed to extract game info.");
    println!("{:?}", game_info)
}
