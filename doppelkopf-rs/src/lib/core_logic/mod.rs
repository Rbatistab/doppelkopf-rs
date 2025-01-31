//! This module has the core logic of the game
//!
//! To use it consider the following schema:
//!   1. Operations, handle the main actions of the game:
//!      * `cheat_sheet_logic`, it gives useful cheat sheets that you can print to your users
//!      * `new_game_logic`, handles the creation of a new game
//!      * `join_game`, handles the request for a user to join an existing game
//!      * `deal_cards`, TBD
//!      * `make_announcement`, TBD
//!      * `play_trick`, TBD
//!   2. Game logic, optional utils to help support the game handling
//!      * `game_state_machine`, TBD
//!
//! # Examples
//! TBD

// Operations logic
pub mod cheat_sheet_logic;
pub mod new_game_logic;

// Game handle logic
pub mod game_state_machine;
pub mod playing_round_state_machine;