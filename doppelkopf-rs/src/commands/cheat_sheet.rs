use log::debug;
use crate::utils::constants::cheat_sheet_strings::GAME_OVERVIEW;

pub enum CheatSheetOptions {
    Contract,
    Trumps,
    Bids,
    SpecialFeatures
}

pub fn print_cheat_sheet() {
    debug!("Printing cheat sheet");
    println!("Mock cheat sheet");
    println!("{GAME_OVERVIEW}");
}