use std::str::from_utf8;

use super::{expansion::Expansion, game_info::GameInfo};

pub fn get_game_cancelled_packet(game_id: u8) -> [u8; 8] {
    [0xf7, 0x33, 0x08, 0x00, game_id, 0x00, 0x00, 0x00]
}

pub fn get_game_announce_packet(game_info: GameInfo) -> [u8; 16] {
    [
        0xf7,
        0x32,
        0x10,
        0x00,
        game_info.game_id.try_into().unwrap(),
        0x00,
        0x00,
        0x00,
        game_info.player_count().try_into().unwrap(),
        0x00,
        0x00,
        0x00,
        game_info.slot_count.try_into().unwrap(),
        0x00,
        0x00,
        0x00,
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
    if response[0] != 0xf7 || response[1] != 0x30 {
        return None;
    }

    let game_id = i32::from_ne_bytes(response[0xc..0xc + 4].try_into().unwrap());
    let name = extract_string(&response[0x14..]);

    let crypt_start = 0x14 + name.len() + 1 + 1;
    let decrypted = decrypt(&response[crypt_start..]);

    let map = extract_string(&decrypted[0xd..]);
    let port = u16::from_ne_bytes(
        response[response.len() - 2..response.len()]
            .try_into()
            .unwrap(),
    );
    let slot_count = i32::from_ne_bytes(
        response[response.len() - 22..response.len() - 18]
            .try_into()
            .unwrap(),
    );
    let current_players = i32::from_ne_bytes(
        response[response.len() - 14..response.len() - 10]
            .try_into()
            .unwrap(),
    );
    let player_slots = i32::from_ne_bytes(
        response[response.len() - 10..response.len() - 6]
            .try_into()
            .unwrap(),
    );

    Some(GameInfo {
        game_id,
        name,
        map,
        port,
        slot_count,
        current_players,
        player_slots,
    })
}

fn extract_string(response: &[u8]) -> String {
    let mut end_index = 0;
    while response[end_index] != 0 {
        end_index += 1;
    }

    from_utf8(&response[0..end_index]).unwrap().to_owned()
}

fn decrypt(data: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut pos = 0;
    let mut mask = 0;
    loop {
        let b = data[pos];
        if b == 0 {
            break;
        };

        if pos % 8 == 0 {
            mask = b;
        } else {
            if (mask & (0x1 << (pos % 8))) == 0 {
                output.push(b - 1);
            } else {
                output.push(b);
            }
        }
        pos += 1;
    }
    output
}

#[cfg(test)]
mod tests {
    use crate::warcraft::{expansion, game_info};

    use super::*;

    #[test]
    fn test_extract_game_info() {
        let response = [
            0xf7, 0x30, 0xa3, 0x00, 0x50, 0x58, 0x33, 0x57, 0x1a, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x00, 0x00, 0x96, 0xde, 0x77, 0x01, 0x4d, 0xc3, 0xad, 0x73, 0x74, 0x6e, 0xc3, 0xad,
            0x20, 0x68, 0x72, 0x61, 0x20, 0x28, 0x44, 0x75, 0x73, 0x61, 0x6e, 0x61, 0x6b, 0x29,
            0x00, 0x00, 0x01, 0x03, 0x49, 0x07, 0x01, 0x01, 0x75, 0x01, 0xd1, 0x55, 0x01, 0xf5,
            0x3b, 0xa7, 0xc7, 0x4d, 0x8b, 0x61, 0x71, 0x73, 0x5d, 0x47, 0x73, 0x6f, 0x85, 0x7b,
            0x65, 0x6f, 0x55, 0x69, 0x73, 0x6f, 0x45, 0x6f, 0x65, 0x5d, 0x29, 0x33, 0x29, 0x53,
            0x67, 0x6f, 0x61, 0x65, 0x55, 0x6f, 0x53, 0x75, 0xa5, 0x73, 0x61, 0x75, 0x69, 0x6f,
            0x6d, 0x6d, 0x1b, 0x65, 0x2f, 0x77, 0x33, 0x79, 0x01, 0x45, 0x6f, 0x75, 0x73, 0x61,
            0x6f, 0x61, 0x6b, 0x01, 0x19, 0x01, 0x7f, 0xcf, 0x15, 0x41, 0x33, 0x87, 0x83, 0xeb,
            0x7b, 0x95, 0xd3, 0xf7, 0x7b, 0x4b, 0x27, 0x05, 0x1f, 0xd1, 0x75, 0x7d, 0x1d, 0x21,
            0x00, 0x02, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02,
            0x00, 0x00, 0x00, 0x22, 0x00, 0x00, 0x00, 0xe0, 0x17,
        ];

        let result = extract_game_info(&response).unwrap();

        assert_eq!(1, result.game_id);
        assert_eq!("Místní hra (Dusanak)", result.name);
        assert_eq!("Maps\\FrozenThrone\\(2)RoadToStratholme.w3x", result.map);
        assert_eq!(6112, result.port);
        assert_eq!(2, result.slot_count);
        assert_eq!(1, result.current_players);
        assert_eq!(2, result.player_slots);
    }

    #[test]
    fn test_get_browse_packet() {
        let browse_packet = [
            0xf7, 0x2f, 0x10, 0x00, 0x50, 0x58, 0x33, 0x57, 0x1a, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ]; // version 26, for some reason ROC, might be a Linux thing

        let expansion = Expansion::ROC;
        let version = 0x1a; // version 26 corresponding to 1.26

        assert_eq!(browse_packet, get_browse_packet(expansion, version));
    }

    #[test]
    fn test_get_game_announce_packet() {
        let game_announce_packet = [
            0xf7, 0x32, 0x10, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00,
            0x00, 0x00,
        ];

        let game_info = GameInfo {
            game_id: 1,
            name: String::from("Test"),
            map: String::from("Test"),
            port: 6112,
            slot_count: 2,
            current_players: 1,
            player_slots: 2,
        };

        assert_eq!(game_announce_packet, get_game_announce_packet(game_info));
    }
}
