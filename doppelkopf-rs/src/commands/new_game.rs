use std::io;
use log::debug;
use game_lib::model::game_state::GameState;
use game_lib::model::player::Player;

pub fn new_game() {
    debug!("Creating new doppelkopf game...");

    let player = get_player();
    debug!("Created new player, {}", player.name);

    let mut game_state=  GameState::new();
    add_mock_players(&mut game_state);
    game_state.add_player(player);

    debug!("Created new game, {}", game_state.id);
    debug!("Game players: {:?}", game_state.players);

    // Get the decks (from lib)
    // Deal the cards (from lib)
    // Allow for announcement (from lib)
    // Play trick (from lib)
    // Evaluate rules (from lib)
}

fn get_player() -> Player {
    debug!("Getting player's name");
    println!("Welcome to doppelkopf!");
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
