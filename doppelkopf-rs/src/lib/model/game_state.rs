use uuid::Uuid;
use crate::model::player::Player;

#[derive(Debug)]
pub struct GameState {
    pub id: String,
    pub players: Vec<Player>
}

impl GameState {
    pub fn new() -> GameState {
        Self {
            id: Uuid::new_v4().to_string(),
            players: Vec::new()
        }
    }

    pub fn add_player(&mut self, player: Player) {
       self.players.push(player);
    }

}