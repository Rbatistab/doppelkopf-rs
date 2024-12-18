mod commands;
mod utils;

use clap::{Parser, Subcommand};
use doppelkopf_cards_lib::suits::SuitType;
use env_logger::{Builder, Env};
use log::debug;
use utils::constants::cli_commands::{
    CLI_ABOUT,
    CLI_LONG_ABOUT,
    PLAY_ABOUT,
    CHEAT_SHEET_ABOUT
};
use crate::commands::cheat_sheet::CheatSheetOption;

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
    #[command(about=PLAY_ABOUT)]
    Play {
        /// Existing game id
        #[arg(short, long)]
        game_id: Option<String>,

        /// Player name
        #[arg(short, long)]
        player_name: Option<String>,

        /// Suit Type
        #[arg(value_enum)]
        #[arg(short, long)]
        suit_type: Option<u8>,

        /// Pack size - only 40 or 48 sizes allowed
        #[arg(short, long)]
        pack_size: Option<u8>
    },

    #[command(about=CHEAT_SHEET_ABOUT)]
    CheatSheet {
        /// Available cheat sheets
        #[arg(short, long)]
        cheat: Option<CheatSheetOption>
    }

}

fn main() {
    let cli = Cli::parse();

    if cli.debug {
        Builder::from_env(Env::default().default_filter_or("debug")).init();
        debug!("Debug mode is on...");
    }

    match &cli.command {
        Some(Commands::Play {game_id, player_name, suit_type, pack_size}) => {
            commands::play::play_doppelkopf();
        },
        Some(Commands::CheatSheet { cheat }) => {
            commands::cheat_sheet::print_cheat_sheet(cheat);
        },
        None => { }
    }
}

// struct NewGameArgs {
//     player_name: String,
//     game_id: Option<String>,
//     suit_type: SuitType
// }
//
// fn get_new_game_arguments(cli: &Cli)  {
//     // game id
//     // player name
//     // suit type
// }

