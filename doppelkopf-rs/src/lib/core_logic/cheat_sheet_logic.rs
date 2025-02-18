//! This logic handles a request for a particular cheat sheet. The game is complex and is useful
//! to have a reminder.

use log::debug;
use crate::model::operations::cheat_sheet_model::{CheatSheetInput, CheatSheetOutput};
use crate::model::types::cheat_sheet::CheatSheetOption;
use crate::utils::constants::cheat_sheet_strings::*;

/// Gets requested cheat sheet
///
/// # Arguments
///
/// * `input` - CheatSheetInput
///
/// # Returns
///
/// CheatSheetOutput containing the requested sheet
///
/// # Examples
/// ```
/// use dppkf_lib::core_logic::cheat_sheet_logic::get_cheat;
/// use dppkf_lib::model::operations::cheat_sheet_model::CheatSheetInput;
/// use dppkf_lib::model::types::cheat_sheet::CheatSheetOption;
/// use dppkf_lib::utils::constants::cheat_sheet_strings::CARD_DEAL;
///
/// let cheat = CheatSheetInput::from(CheatSheetOption::CardDeal);
/// let sheet = get_cheat(cheat);
/// assert_eq!(sheet.cheat, CARD_DEAL.to_string())
/// ```
pub fn get_cheat(input: CheatSheetInput) -> CheatSheetOutput {
    debug!("Getting cheat sheet from core logic...");
    match input.cheat_sheet {
        CheatSheetOption::GameOverview => {
            CheatSheetOutput::from(GAME_OVERVIEW.to_string())
        },
        CheatSheetOption::CardDeal => {
            CheatSheetOutput::from(CARD_DEAL.to_string())
        },
        CheatSheetOption::Contracts => {
            CheatSheetOutput::from(CONTRACTS.to_string())
        },
        CheatSheetOption::Tricks => {
            CheatSheetOutput::from(TRICKS.to_string())
        },
        CheatSheetOption::Bids => {
            CheatSheetOutput::from(BIDS.to_string())
        },
        CheatSheetOption::Trumps => {
            CheatSheetOutput::from(TRUMPS.to_string())
        },
        CheatSheetOption::Rules => {
            CheatSheetOutput::from(RULES.to_string())
        },
        CheatSheetOption::SpecialFeatures => {
            CheatSheetOutput::from(SPECIAL_FEATURES.to_string())
        },
        CheatSheetOption::Scoring => {
            CheatSheetOutput::from(SCORING.to_string())
        }
    }
}