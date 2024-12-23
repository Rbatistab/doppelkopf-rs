use std::io;
use log::debug;
use doppelkopf_cards_lib::suits::SuitType;
use game_lib::model::game_state::GameState;
use game_lib::model::player::Player;

pub fn new_game_cli(player_name: &Option<String>, suit_type: &Option<SuitType>, pack_size: &Option<u8>) {
    debug!("Creating new doppelkopf game...");
    debug!("Provided player_name: {:?}", player_name);
    debug!("Provided suit_type: {:?}", suit_type);
    debug!("Provided pack_size: {:?}", pack_size);

    println!("Welcome to doppelkopf! Let's start a new game");

    let mut player: Player;

    match player_name {
        Some(name) => {
            debug!("Player name provided on 'new-game'");
        }
        None => {
            debug!("No player name provided on 'new-game'");
        }
    }

    // if Some(player_name).is_some() {
    //     debug!("Player name provided on 'new-game'");
    //     // player = Player::from_name(player_name.trim().to_string());
    // } else {
    //     debug!("No player name provided on 'new-game'");
    //     // player = get_player_from_cli();
    // }

    // debug!("Created new player, {}", player.name);


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
}

fn get_player_from_cli() -> Player {
    debug!("Getting player's name");
    println!("What's your name? ");

    let mut player_name = String::new();
    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read your name");

    Player::from_name(player_name.trim().to_string())
}

fn add_mock_players(game_state: &mut GameState) {
    let mock_player_names = ["Sabina", "Jasper", "Matthias"];

    for name in mock_player_names.iter() {
        game_state.add_player(Player::from_name(name.parse().unwrap()));
    }

}

#[cfg(test)]
mod new_game_cli_tests {
    #[test]
    fn test_no_arguments_provided() {
        // TBD
       assert_eq!(1,1) ;
    }
}
