use log::debug;
use crate::model::types::cheat_sheet::CheatSheetOption;
use crate::utils::constants::cheat_sheet_strings::*;

pub fn get_cheat_sheet(cheat: &Option<CheatSheetOption>) -> &'static str {
    debug!("Getting cheat sheet from core logic...");
    match *cheat {
        Some(CheatSheetOption::GameOverview) => {
            GAME_OVERVIEW
        },
        Some(CheatSheetOption::CardDeal) => {
            CARD_DEAL
        },
        Some(CheatSheetOption::Contracts) => {
            CONTRACTS
        },
        Some(CheatSheetOption::Tricks) => {
            TRICKS
        },
        Some(CheatSheetOption::Trumps) => {
            TRUMPS
        },
        Some(CheatSheetOption::Rules) => {
            RULES
        },
        Some(CheatSheetOption::SpecialFeatures) => {
            SPECIAL_FEATURES
        },
        Some(CheatSheetOption::Scoring) => {
            SCORING
        },
        None => {
            GAME_OVERVIEW
        }
    }
}