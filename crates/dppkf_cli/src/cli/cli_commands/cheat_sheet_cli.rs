use dppkf_lib::core_logic::cheat_sheet_logic::get_cheat;
use dppkf_lib::model::types::cheat_sheet::CheatSheetOption;
use log::debug;
use dppkf_lib::model::operations::cheat_sheet_model::CheatSheetInput;
use crate::CheatSheetOptionCli;

pub fn print_cheat_sheet(cheat_arg: &Option<CheatSheetOptionCli>) {
    debug!("Printing cheat sheet...");
    debug!("Cheat cheat requested with flag: {:?}", cheat_arg);

    match cheat_arg {
        Some(cheat) => {
            let cheat_option = match cheat {
                CheatSheetOptionCli::GameOverview => CheatSheetOption::GameOverview,
                CheatSheetOptionCli::CardDeal => CheatSheetOption::CardDeal,
                CheatSheetOptionCli::Contracts => CheatSheetOption::Contracts,
                CheatSheetOptionCli::Tricks => CheatSheetOption::Tricks,
                CheatSheetOptionCli::Bids => CheatSheetOption::Bids,
                CheatSheetOptionCli::Trumps => CheatSheetOption::Trumps,
                CheatSheetOptionCli::Rules => CheatSheetOption::Rules,
                CheatSheetOptionCli::SpecialFeatures => CheatSheetOption::SpecialFeatures,
                CheatSheetOptionCli::Scoring => CheatSheetOption::Scoring,
            };
            let cheat_string = get_cheat(CheatSheetInput::from(cheat_option)).cheat;
            println!("{cheat_string}");
        },
        None => {
            // Should make an interactive shell here instead?
            let cheat_string = get_cheat(CheatSheetInput::default()).cheat;
            println!("{cheat_string}");
        },
    }
}