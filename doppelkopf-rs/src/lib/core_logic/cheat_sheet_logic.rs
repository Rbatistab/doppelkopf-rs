use log::debug;
use crate::model::types::cheat_sheet::CheatSheetOption;
use crate::utils::constants::cheat_sheet_strings::*;

pub fn get_cheat_sheet(cheat: &Option<CheatSheetOption>) -> &'static str {
    debug!("Getting cheat sheet from core logic...");
    match *cheat {
        Some(CheatSheetOption::GameOverview) => {
            return GAME_OVERVIEW;
        },
        Some(CheatSheetOption::CardDeal) => {
            return CARD_DEAL;
        },
        Some(CheatSheetOption::Contracts) => {
            return CONTRACTS;
        },
        Some(CheatSheetOption::Tricks) => {
            return TRICKS;
        },
        Some(CheatSheetOption::Trumps) => {
            return TRUMPS;
        },
        Some(CheatSheetOption::Rules) => {
            return RULES;
        },
        Some(CheatSheetOption::SpecialFeatures) => {
            return SPECIAL_FEATURES;
        },
        Some(CheatSheetOption::Scoring) => {
            return SCORING;
        },
        None => {
            return GAME_OVERVIEW;
        }
    }
}