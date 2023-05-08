use std::{
    io,
    net::{SocketAddrV4, UdpSocket},
    time::Duration,
};

use crate::warcraft::{
    expansion::Expansion,
    game_info::GameInfo,
    query_protocol::{extract_game_info, get_browse_packet, get_game_announce_packet},
};

pub struct InfoClient {
    socket: UdpSocket,
    last_game_info_packet: Option<Vec<u8>>,
}

impl InfoClient {
    pub fn new(port: u16) -> Self {
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", port)).expect("Failed to bind UDP socket.");
        socket.set_broadcast(true).unwrap();
        socket.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
        Self {
            socket,
            last_game_info_packet: None,
        }
    }

    pub fn start(&mut self, address: String) {
        let addr: SocketAddrV4 = address.parse().unwrap_or_else(|_| -> SocketAddrV4 {
            println!("Defaulting to port 6112.");
            SocketAddrV4::new(address.parse().expect("Failed to parse IP address"), 6112)
        });

        loop {
            self.send_browse_packet(&addr);
            self.process_responses();
        }
    }

    fn send_browse_packet(&self, addr: &SocketAddrV4) {
        // println!("Sending a browse packet to {}.", addr);
        self.socket
            .send_to(&get_browse_packet(Expansion::ROC, 26), addr)
            .expect(&format!("Failed to send browse packet to {}", addr));
    }

    fn advertise_server(&self, game_info: &GameInfo) {
        let announce_packet = get_game_announce_packet(game_info);
        // println!("Sending an announce packet.");
        self.socket
            .send_to(
                &announce_packet,
                "255.255.255.255:6112".parse::<SocketAddrV4>().unwrap(),
            )
            .unwrap();
    }

    fn process_responses(&mut self) {
        // println!("Processing responses.");
        let mut buf = [0; 256];
        loop {
            match self.socket.recv_from(&mut buf) {
                Ok((received, addr)) => {
                    // print!("Received from {}: {} - ", addr, received);
                    // for i in 0..received {
                    //     print!("0x{:x} ", buf[i])
                    // }
                    // println!();

                    // game info packet
                    if buf[0] == 0xf7 && buf[1] == 0x30 {
                        // println!("Game info received.");
                        let game_info = extract_game_info(&buf[..received])
                            .expect("Failed to extract game info.");
                        self.advertise_server(&game_info);
                        self.last_game_info_packet = Some(buf[..received].to_vec());
                    }

                    // browse packet
                    if buf[0] == 0xf7 && buf[1] == 0x2f {
                        // println!("Browse packet received.");
                        if let Some(packet) = &self.last_game_info_packet {
                            self.socket.send_to(packet, addr).unwrap();
                        }
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // println!("No more responses found.");
                    break;
                }
                Err(e) => panic!("encountered IO error: {e}"),
            }
        }
    }
}
