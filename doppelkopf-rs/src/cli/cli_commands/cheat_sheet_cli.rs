use dppkf_lib::core_logic::cheat_sheet_logic::get_cheat_sheet;
use dppkf_lib::model::types::cheat_sheet::CheatSheetOption;
use log::debug;

pub fn print_cheat_sheet(cheat: &Option<CheatSheetOption>) {
    debug!("Printing cheat sheet...");
    debug!("Cheat cheat requested with flag: {:?}", cheat);
    let cheat_sheet = get_cheat_sheet(cheat);
    println!("{cheat_sheet}");
}

// #[cfg(test)]
// mod cheat_sheet_tests {
//     use super::*;
//     use std::io::{self,Write};
//     use std::string::String;
//     use dppkf_lib::utils::constants::cheat_sheet_strings::GAME_OVERVIEW;
//
//     #[test]
//     fn test_print_cheat_sheet() {
//         let mut cli_output = Vec::new();
//         {
//             // let mut stdout = std::io::stdout();
//             // let _ = stdout.write_all(b"");
//             // let _ = stdout.flush();
//             let mut handle = io::Cursor::new(&mut cli_output);
//             print_cheat_sheet(&None);
//             handle.flush().unwrap();
//         }
//
//         let output_str = String::from_utf8(cli_output).unwrap();
//
//         assert!(output_str.contains(GAME_OVERVIEW));
//
//     }
// }