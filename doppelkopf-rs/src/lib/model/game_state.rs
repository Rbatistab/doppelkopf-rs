use uuid::Uuid;
use doppelkopf_cards_lib::decks::Deck;
use doppelkopf_cards_lib::suits::SuitType;
use crate::model::player::Player;

#[derive(Debug)]
pub struct GameState {
    pub id: String,
    pub players: Vec<Player>,
    pub deck: Deck
}

impl GameState {
    pub fn new() -> GameState {
        Self {
            id: Uuid::new_v4().to_string(),
            players: Vec::new(),
            deck: Deck::from(SuitType::French)
        }
    }

    pub fn add_player(&mut self, player: Player) {
       self.players.push(player);
    }

}