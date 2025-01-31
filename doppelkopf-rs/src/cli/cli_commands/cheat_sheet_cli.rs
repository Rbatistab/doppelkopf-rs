use dppkf_lib::core_logic::cheat_sheet_logic::get_cheat;
use dppkf_lib::model::types::cheat_sheet::CheatSheetOption;
use log::debug;
use dppkf_lib::model::operations::cheat_sheet_model::CheatSheetInput;

pub fn print_cheat_sheet(cheat_arg: &Option<CheatSheetOption>) {
    debug!("Printing cheat sheet...");
    debug!("Cheat cheat requested with flag: {:?}", cheat_arg);

    match cheat_arg {
        Some(cheat) => {
            let cheat_string = get_cheat(CheatSheetInput::from(cheat.clone())).cheat;
            println!("{cheat_string}");
        },
        None => {
            let cheat_string = get_cheat(CheatSheetInput::default()).cheat;
            println!("{cheat_string}");
        },
    }

}