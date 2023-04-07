#[derive(Debug)]
pub struct GameInfo {
    pub game_id: i32,
    pub name: String,
    pub map: String,
    pub port: u16,
    pub slot_count: i32,
    pub current_players: i32,
    pub player_slots: i32,
}

impl GameInfo {
    pub fn player_count(&self) -> i32 {
        self.slot_count - self.player_slots + self.current_players
    }
}
