//! Creates a new game and stores it on the database. Also will create a new UUID for the player creating the game and return it to the FE. (All UUIDs are Backend-generated).

use log::debug;
use uuid::Uuid;
use crate::model::operations::new_game_model::NewGameLogicArgs;

/// Creates a new CLI game, it will take parameters from the CLI command `new-game` or it will
/// capture them dynamically with the user.
///
/// # Fields
/// * `player_name` - (Optional) Name of player creating new game
/// * `suit_type` - (Optional) Suit Type
/// * `pack_size` - (Optional) Pack size
pub fn get_new_game(args: NewGameLogicArgs) -> String {
    debug!("Creating new game with args:\n{:?}", args);
    let game_id = Uuid::new_v4().to_string();
    // set this id to the game state

    // let mut game_state=  GameState::new();
    // add_mock_players(&mut game_state);
    // game_state.add_player(player);

    // debug!("Created new game, {}", game_state.id);
    // debug!("Game players: {:?}", game_state.players);

    // Get the decks (from lib)
    // Deal the cards (from lib)
    // Allow for announcement (from lib)
    // Play trick (from lib)
    // Evaluate rules (from lib)
    return game_id;
}

// fn add_mock_players(game_state: &mut GameState) {
//     let mock_player_names = ["Sabina", "Jasper", "Matthias"];
//
//     for name in mock_player_names.iter() {
//         game_state.add_player(Player::from_name(name.parse().unwrap()));
//     }
// }

#[cfg(test)]
mod new_game_tests {
    #[test]
    fn test_get_new_game() {
        // Mock test
        assert_eq!(1,1);
    }
}