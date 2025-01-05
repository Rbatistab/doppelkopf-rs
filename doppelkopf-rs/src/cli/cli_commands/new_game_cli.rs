//! Hanldes creation of a new game on CLI

use crate::cli::cli_utils::constants::new_game_cli_constants::{FAILED_GET_PLAYER_NAME_STR, FAILED_GET_SUIT_TYPE_STR, GET_PACK_SIZE_STR, GET_PLAYER_NAME_STR, GET_SUIT_TYPES_STR, INVALID_PACK_SIZE_STR, INVALID_SUIT_TYPE_STR};
use std::io;
use log::debug;
use doppelkopf_cards_lib::suits::SuitType;
use dppkf_lib::core_logic::new_game_logic::get_new_game;
use dppkf_lib::model::operations::new_game_model::NewGameLogicArgs;
use dppkf_lib::model::types::player::Player;

/// Creates a new CLI game, it will take parameters from the CLI command `new-game` or it will
/// capture them dynamically with the user.
///
/// # Fields
/// * `player_name` - (Optional) Name of player creating new game
/// * `suit_type` - (Optional) Suit Type
/// * `pack_size` - (Optional) Pack size
pub fn new_game_cli(player_name: &Option<String>, suit_type: &Option<SuitType>, pack_size: &Option<u8>) {
    debug!("Creating new doppelkopf game...");
    debug!("Provided player_name: {:?}", player_name);
    debug!("Provided suit_type: {:?}", suit_type);
    debug!("Provided pack_size: {:?}", pack_size);

    println!("Welcome to doppelkopf! Let's start a new game");

    let mut new_game_args = NewGameLogicArgs::new();

    match player_name {
        Some(name) => {
            debug!("Player name provided on 'new-game'");
            new_game_args.add_player(Player::human_player_from_name(name.to_string()));
        }
        None => {
            debug!("No player name provided on 'new-game'");
            new_game_args.add_player(get_player_from_cli());
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
            new_game_args.set_pack_size(size.clone());
        }
        None => {
            debug!("No pack size provided on 'new-game");
            new_game_args.set_pack_size(get_pack_size_from_cli());
        }
    }

    debug!("Added player, suit type and pack size to new_game_args, {:?}", new_game_args);

    let new_game_id = get_new_game(new_game_args);
    debug!("New game created! Id: {new_game_id}")

}

fn get_player_from_cli() -> Player {
    debug!("Getting player's name...");
    println!("{GET_PLAYER_NAME_STR}");

    let mut player_name = String::new();
    io::stdin()
        .read_line(&mut player_name)
        .expect(FAILED_GET_PLAYER_NAME_STR);

    Player::human_player_from_name(player_name.trim().to_string())
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

pub fn get_pack_size_from_cli() -> u8 {
    debug!("Getting pack size...");

    loop {
        println!("{GET_PACK_SIZE_STR}");

        let mut suit_type_input = String::new();
        io::stdin()
            .read_line(&mut suit_type_input)
            .expect(FAILED_GET_SUIT_TYPE_STR);

        match suit_type_input.trim().parse::<u32>() {
            Ok(1) => return 48,
            Ok(2) => return 40,
            Ok(_) => {
                println!("{INVALID_PACK_SIZE_STR}");
                continue;
            },
            Err(_) => {
                println!("{INVALID_PACK_SIZE_STR}");
                continue;
            }
        }
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
