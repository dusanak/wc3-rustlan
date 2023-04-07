use std::str::from_utf8;

use super::{expansion::Expansion, game_info::GameInfo};

pub fn get_game_cancelled_packet(game_id: u8) -> [u8; 8] {
    [0xf7, 0x33, 0x08, 0x00, game_id, 0x00, 0x00, 0x00]
}

pub fn get_game_announce_packet(game_info: GameInfo) -> [u8; 16] {
    [
        0xf7, 0x32, 0x10, 0x00, game_info.game_id.try_into().unwrap(), 0x00, 0x00, 0x00, game_info.player_count().try_into().unwrap(),
        0x00, 0x00, 0x00, game_info.slot_count.try_into().unwrap(), 0x00, 0x00, 0x00,
    ]
}

pub fn get_browse_packet(expansion: Expansion, version: u8) -> [u8; 16] {
    match expansion {
        Expansion::ROC => [
            0xf7, 0x2f, 0x10, 0x00, 0x50, 0x58, 0x33, 0x57, version, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
        Expansion::TFT => [
            0xf7, 0x2f, 0x10, 0x00, 0x33, 0x52, 0x41, 0x57, version, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
    }
}

pub fn extract_game_info(response: &[u8]) -> Option<GameInfo> {  
    if response[0] != 0xf7 || response[1] != 0x32 {
        return None;
    }

    let game_id = i32::from_ne_bytes(response[0xc..0xc + 4].try_into().unwrap());
    let name = extract_string(&response[0x14..]);

    let crypt_start = 0x14 + name.len() + 1 + 1;
    let decrypted = decrypt(&response[crypt_start..]);

    let map = extract_string(&decrypted);
    let port = u16::from_ne_bytes(response[response.len() - 2..response.len()].try_into().unwrap());
    let slot_count = i32::from_ne_bytes(response[response.len() - 22..response.len() - 18].try_into().unwrap());
    let current_players = i32::from_ne_bytes(response[response.len() - 14..response.len() - 10].try_into().unwrap());
    let player_slots = i32::from_ne_bytes(response[response.len() - 10..response.len() - 6].try_into().unwrap());


    Some(
        GameInfo {
            game_id,
            name,
            map,
            port,
            slot_count,
            current_players,
            player_slots
        }
    )
}

fn extract_string(response: &[u8]) -> String {
    let mut end_index = 0;
    while response[end_index] != 0 {
        end_index += 1;
    };

    from_utf8(&response[0..end_index]).unwrap().to_owned()
}

fn decrypt(data: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut pos = 0;
    let mut mask = 0;
    loop
    {
        let b = data[pos];
        if b == 0 { 
            break 
        };

        if pos % 8 == 0 {
            mask = b;
        }
        else {
            if (mask & (0x1 << (pos % 8))) == 0 {
                output.push(b - 1);
            }
            else{
                output.push(b);
            }
        }
        pos += 1;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_game_info() {

        let arr = [
            0xf7, 0x32, 0x10, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00
        ];

        let result = extract_game_info(&arr);
        match result {
            Some(result) => println!("{:?}", result),
            None => println!("Extraction failed!"),
        }
    }
    

}