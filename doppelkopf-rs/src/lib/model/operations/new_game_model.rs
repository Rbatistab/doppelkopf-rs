use doppelkopf_cards_lib::suits::SuitType;
use crate::model::types::player::Player;

/// Fields for new_game operation
///
/// # Fields
/// * `player` - Player creating the game
/// * `suit_type` - Suit type of the new game
/// * `pack_size` - Pack size of new game, 40 or 48
#[derive(Debug)]
pub struct NewGameLogicArgs {
    player: Player,
    suit_type: SuitType,
    pack_size: u8,
    // game state handler (DDB, S3, ram)
}

impl NewGameLogicArgs {
    pub fn new() -> NewGameLogicArgs {
        NewGameLogicArgs {
            player: Player::new(),
            suit_type: SuitType::French,
            pack_size: 48,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.player = player;
    }

    pub fn set_suit_type(&mut self, suit_type: SuitType)  {
        self.suit_type = suit_type;
    }

    pub fn set_pack_size(&mut self, pack_size: u8)  {
        self.pack_size = pack_size;
    }
}

#[cfg(test)]
mod new_game_logic_args_test {
    use super::*;

    #[test]
    fn test_new_game_logic_args_new_fn() {
        let new = NewGameLogicArgs::new();
        assert_eq!(new.player, Player::new());
        assert_eq!(new.suit_type, SuitType::French);
        assert_eq!(new.pack_size, 48);
    }

}