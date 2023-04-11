use std::net::{UdpSocket, SocketAddrV4};

use crate::warcraft::{query_protocol::{get_browse_packet, extract_game_info}, expansion::Expansion, game_info::GameInfo};

pub struct InfoClient {
    socket: UdpSocket
}

impl InfoClient {
    pub fn new () -> Self {
        let socket = UdpSocket::bind("0.0.0.0:6112").expect("Failed to bind UDP socket.");
        Self { socket }
    }

    pub fn get_game_info(&self, address: &str) -> GameInfo {
        let addr: SocketAddrV4 = address.parse().unwrap_or_else(|_| -> SocketAddrV4 {
            println!("Defaulting to port 6112.");
            SocketAddrV4::new(address.parse().expect("Failed to parse IP address"), 6112)
        });
        
        println!("Sending a browse packet to {}.", addr);
        self.socket
            .send_to(&get_browse_packet(Expansion::ROC, 26), addr)
            .expect(&format!("Failed to send browse packet to {}", addr));
    
        println!("Waiting for game info.");
        let mut buf = [0; 256];
        let (received, _) = self.socket.recv_from(&mut buf).expect("Failed to receive game info.");
    
        extract_game_info(&buf[..received]).expect("Failed to extract game info.")
    }
}