//! Handles creation of a new game on CLI

use crate::cli::cli_utils::constants::new_game_cli_constants::{
    GET_PACK_SIZE_STR,
    GET_PLAYER_NAME_STR,
    GET_SUIT_TYPES_STR,
    INVALID_PACK_SIZE_STR,
    INVALID_SUIT_TYPE_STR,
    FAILED_GET_PACK_SIZE_STR,
    FAILED_GET_PLAYER_NAME_STR,
    FAILED_GET_SUIT_TYPE_STR
};
use std::io;
use log::debug;
use doppelkopf_cards_lib::deck::PackSize;
use doppelkopf_cards_lib::suits::SuitType;
use dppkf_lib::core_logic::game_state_machine::GameStateMachine;
use dppkf_lib::core_logic::new_game_logic::get_new_game;
use dppkf_lib::model::operations::new_game_model::NewGameArgs;
use dppkf_lib::model::types::player::{Player, PlayerType};

/// Creates a new CLI game, it will take parameters from the CLI command `new-game` or it will
/// capture them dynamically with the user.
///
/// # Fields
/// * `player_name` - (Optional) Name of player creating new game
/// * `suit_type` - (Optional) Suit Type
/// * `pack_size` - (Optional) Pack size
pub fn new_game_cli(player_name: &Option<String>, suit_type: &Option<SuitType>, pack_size: &Option<u8>) {
    debug!("Provided player_name: {:?}", player_name);
    debug!("Provided suit_type: {:?}", suit_type);
    debug!("Provided pack_size: {:?}", pack_size);

    debug!("Creating new doppelkopf game...");

    let game_state_machine = GameStateMachine::new();
    debug!("Created new game state machine: {:?}", game_state_machine);


    println!("Welcome to doppelkopf! Let's start a new game");

    let mut new_game_args = NewGameArgs::new();

    match player_name {
        Some(name) => {
            debug!("Player name provided on 'new-game'");
            new_game_args.set_player(Player::from(name.to_string(), PlayerType::Human));
        }
        None => {
            debug!("No player name provided on 'new-game'");
            new_game_args.set_player(get_player_from_cli());
        }
    }

    match suit_type {
        Some(suit_type) => {
            debug!("Suit type provided on 'new-game'");
            new_game_args.set_suit_type(suit_type.clone());
        }
        None => {
            debug!("No suit type provided on 'new-game'");
            new_game_args.set_suit_type(get_suit_type_from_cli());
        }
    }

    match pack_size {
        Some(size) => {
            debug!("Pack size provided on 'new-game");
            let size = match size {
                40 => PackSize::Forty,
                48 => PackSize::FortyEight,
                _ => PackSize::FortyEight
            };
            new_game_args.set_pack_size(size);
        }
        None => {
            debug!("No pack size provided on 'new-game");
            new_game_args.set_pack_size(get_pack_size_from_cli());
        }
    }

    debug!("Added player, suit type and pack size to new_game_args, {:?}", new_game_args);

    let new_game_id = get_new_game(new_game_args);
    debug!("New game created! Id: {new_game_id}");

    println!("Your new game id is: {new_game_id}");

    // Now should jump to a game started

}

fn get_player_from_cli() -> Player {
    debug!("Getting player's name...");
    println!("{GET_PLAYER_NAME_STR}");

    let mut player_name = String::new();
    io::stdin()
        .read_line(&mut player_name)
        .expect(FAILED_GET_PLAYER_NAME_STR);

    Player::from(player_name.trim().to_string(), PlayerType::Human)
}

fn get_suit_type_from_cli() -> SuitType {
    debug!("Getting suit type...");

    loop {
        println!("{GET_SUIT_TYPES_STR}");

        let mut suit_type_input = String::new();
        io::stdin()
            .read_line(&mut suit_type_input)
            .expect(FAILED_GET_SUIT_TYPE_STR);

        match suit_type_input.trim().parse::<u32>() {
            Ok(1) => return SuitType::French,
            Ok(2) => return SuitType::German,
            Ok(_) => {
                println!("{INVALID_SUIT_TYPE_STR}");
                continue;
            },
            Err(_) => {
                println!("{INVALID_SUIT_TYPE_STR}");
                continue;
            }
        }
    }
}

pub fn get_pack_size_from_cli() -> PackSize {
    debug!("Getting pack size...");

    loop {
        println!("{GET_PACK_SIZE_STR}");

        let mut suit_type_input = String::new();
        io::stdin()
            .read_line(&mut suit_type_input)
            .expect(FAILED_GET_SUIT_TYPE_STR);

        match suit_type_input.trim().parse::<u32>() {
            Ok(1) => return PackSize::FortyEight,
            Ok(2) => return PackSize::Forty,
            Ok(_) => {
                println!("{INVALID_PACK_SIZE_STR}");
                continue;
            },
            Err(_) => {
                println!("{FAILED_GET_PACK_SIZE_STR}");
                continue;
            }
        }
    }
}
