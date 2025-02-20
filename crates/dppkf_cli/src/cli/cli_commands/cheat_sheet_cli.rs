use inquire::{InquireError, Select};
use dppkf_lib::core_logic::cheat_sheet_logic::get_cheat;
use dppkf_lib::model::types::cheat_sheet::CheatSheetOption;
use log::{debug, error};
use dppkf_lib::model::operations::cheat_sheet_model::CheatSheetInput;
use crate::CheatSheetOptionCli;

pub fn print_cheat_sheet(cheat_arg: &Option<CheatSheetOptionCli>) {
    debug!("Printing cheat sheet...");
    debug!("Cheat cheat requested with flag: {:?}", cheat_arg);

    match cheat_arg {
        Some(cheat) => {
            let cheat_option = get_cheat_sheet_option_from_cli_option(cheat);
            let cheat_string = get_cheat(CheatSheetInput::from(cheat_option)).cheat;
            println!("{cheat_string}");
        },
        None => {
            match select_cheat_option() {
                Ok(option) => {
                    let cheat_string = get_cheat(CheatSheetInput::from(option)).cheat;
                    println!("{cheat_string}");
                },
                Err(e) => {
                    error!("An error happened while getting the desired cheat{}", e);
                    eprintln!("An error happened while getting the desired cheat{}", e)
                }
            }
        },
    }
}


fn get_cheat_sheet_option_from_cli_option(cheat: &CheatSheetOptionCli) -> CheatSheetOption {
    debug!("Entering interactive prompt to get desired cheat...");
    match cheat {
        CheatSheetOptionCli::GameOverview => CheatSheetOption::GameOverview,
        CheatSheetOptionCli::CardDeal => CheatSheetOption::CardDeal,
        CheatSheetOptionCli::Contracts => CheatSheetOption::Contracts,
        CheatSheetOptionCli::Tricks => CheatSheetOption::Tricks,
        CheatSheetOptionCli::Bids => CheatSheetOption::Bids,
        CheatSheetOptionCli::Trumps => CheatSheetOption::Trumps,
        CheatSheetOptionCli::Rules => CheatSheetOption::Rules,
        CheatSheetOptionCli::SpecialFeatures => CheatSheetOption::SpecialFeatures,
        CheatSheetOptionCli::Scoring => CheatSheetOption::Scoring,
    }
}

fn select_cheat_option() -> Result<CheatSheetOption, InquireError> {
    let cheats = vec![
        CheatSheetOption::GameOverview,
        CheatSheetOption::CardDeal,
        CheatSheetOption::Contracts,
        CheatSheetOption::Tricks,
        CheatSheetOption::Bids,
        CheatSheetOption::Trumps,
        CheatSheetOption::Rules,
        CheatSheetOption::SpecialFeatures,
        CheatSheetOption::Scoring,
    ];

    Select::new("Select a cheat: ", cheats)
        .with_help_message("Use ↑↓ arrows to navigate, enter to select")
        .prompt()
}