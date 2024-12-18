use clap::ValueEnum;
use log::debug;
use crate::utils::constants::cheat_sheet_strings::{CARD_DEAL, CONTRACTS, GAME_OVERVIEW, RULES, SCORING, SPECIAL_FEATURES, TRICKS, TRUMPS};

#[derive(Clone, ValueEnum)]
pub enum CheatSheetOption {
    GameOverview,
    CardDeal,
    Contracts,
    Tricks,
    Trumps,
    Rules,
    SpecialFeatures,
    Scoring
}

pub fn print_cheat_sheet(cheat: &Option<CheatSheetOption>) {
    debug!("Printing cheat sheet");
    match *cheat {
        Some(CheatSheetOption::GameOverview) => {
            println!("{GAME_OVERVIEW}");
        },
        Some(CheatSheetOption::CardDeal) => {
            println!("{CARD_DEAL}");
        },
        Some(CheatSheetOption::Contracts) => {
            println!("{CONTRACTS}");
        },
        Some(CheatSheetOption::Tricks) => {
            println!("{TRICKS}");
        },
        Some(CheatSheetOption::Trumps) => {
            println!("{TRUMPS}");
        },
        Some(CheatSheetOption::Rules) => {
            println!("{RULES}");
        },
        Some(CheatSheetOption::SpecialFeatures) => {
            println!("{SPECIAL_FEATURES}");
        },
        Some(CheatSheetOption::Scoring) => {
            println!("{SCORING}");
        },
        None => {
            println!("{GAME_OVERVIEW}");
        }
    }
}