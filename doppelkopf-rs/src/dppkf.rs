mod commands;
mod utils;

use clap::{Parser, Subcommand};
use doppelkopf_cards_lib::suits::SuitType;
use env_logger::{Builder, Env};
use log::debug;
use utils::constants::cli_commands::{
    CLI_ABOUT,
    CLI_LONG_ABOUT,
    JOIN_GAME_ABOUT,
    NEW_GAME_ABOUT,
    CHEAT_SHEET_ABOUT
};
use crate::commands::cheat_sheet_cli::CheatSheetOption;

#[derive(Parser)]
#[command(version)]
#[command(about = CLI_ABOUT, long_about = CLI_LONG_ABOUT)]
struct Cli {
    /// Sets debug mode on
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    #[command(about=NEW_GAME_ABOUT)]
    NewGame {
        /// Player name
        #[arg(short = 'n', long)]
        player_name: Option<String>,

        /// Suit Type
        #[arg(value_enum)]
        #[arg(short, long)]
        suit_type: Option<SuitType>,

        /// Pack Size [possible values: 40, 48]
        #[arg(short = 'p', long)]
        #[arg(value_parser = valid_pack_size)]
        pack_size: Option<u8>
    },

    #[command(about=JOIN_GAME_ABOUT)]
    JoinGame {
        /// Existing game id
        #[arg(short, long)]
        game_id: Option<String>,

        /// Player name
        #[arg(short, long)]
        player_name: Option<String>,
    },

    #[command(about=CHEAT_SHEET_ABOUT)]
    CheatSheet {
        /// Available cheat sheets
        #[arg(short, long)]
        cheat: Option<CheatSheetOption>
    }

}

fn valid_pack_size(s: &str) -> Result<u8, String> {
    let invalid_pack_size_message = "Pack size can only be '40' or '48'.";
    let size: u8 = s.parse().map_err(|_| invalid_pack_size_message)?;
    if size == 40 || size == 48 {
        Ok(size)
    } else {
        Err(String::from(invalid_pack_size_message))
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.debug {
        Builder::from_env(Env::default().default_filter_or("debug")).init();
        debug!("Debug mode is on...");
    }

    match &cli.command {
        Some(Commands::NewGame {player_name, suit_type, pack_size}) => {
            commands::new_game_cli::new_game_cli(player_name, suit_type, pack_size);
        },
        Some(Commands::JoinGame { game_id, player_name }) => {
            commands::join_game_cli::join_game();
        }
        Some(Commands::CheatSheet { cheat }) => {
            commands::cheat_sheet_cli::print_cheat_sheet(cheat);
        },
        None => { }
    }
}