mod warcraft {
    pub mod expansion;
    pub mod game_info;
    pub mod query_protocol;
}

mod net {
    pub mod info_client;
}

use std::{
    env,
};

use net::info_client::InfoClient;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Not enough parameters passed.");
        return;
    }

    let info_client = InfoClient::new();
    let game_info = info_client.get_game_info(&args[1]);
    println!("{:?}", game_info)
}
