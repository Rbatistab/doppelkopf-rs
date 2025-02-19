//! Card rank implementations for both French and German playing cards.
//!
//! This module provides representations and conversions for card ranks
//! in both French (international) and German card systems.

use const_format::concatcp;
use crate::utils::constants::{BOLD, RESET};
use crate::ranks::Rank::{FrenchRank, GermanRank};

/// Represents French card ranks
#[derive(Debug, PartialEq)]
pub enum FrenchCardRank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

/// Represents German card ranks
#[derive(Debug, PartialEq)]
pub enum GermanCardRank {
    Sieben,
    Acht,
    Neun,
    Zehn,
    Unter,
    Ober,
    Koing,
    Ass,
}

/// Represents the rank of a card, for either German or French cards
#[derive(Debug, PartialEq)]
pub enum Rank {
    FrenchRank(FrenchCardRank),
    GermanRank(GermanCardRank)
}

impl Rank {
    /// Converts a rank to its string representation.
    ///
    /// Returns a static string with a consistent 2-character width
    /// (padded with space where needed).
    ///
    /// # Examples
    ///
    /// This example uses `BOLD` and `RESET` formating because the ranks are bolded
    ///
    /// ```
    /// use doppelkopf_cards_lib::ranks::Rank;
    /// use doppelkopf_cards_lib::ranks::FrenchCardRank::Two;
    /// use doppelkopf_cards_lib::ranks::GermanCardRank::Sieben;
    ///
    /// let french_rank = Rank::FrenchRank(Two).to_str();
    /// let german_rank = Rank::GermanRank(Sieben).to_str();
    ///
    /// const BOLD: &str = "\x1b[1m";
    /// const RESET: &str = "\x1b[0m";
    /// let two = format!("{}2 {}", BOLD, RESET);
    /// let seven = format!("{}7 {}", BOLD, RESET);
    ///
    /// assert_eq!(french_rank, &two);
    /// assert_eq!(german_rank, &seven);
    /// ```
    pub fn to_str(&self) -> &'static str {
        match self {
            FrenchRank(FrenchCardRank::Two) => { concatcp!(BOLD, "2 ", RESET) },
            FrenchRank(FrenchCardRank::Three) => { concatcp!(BOLD, "3 ", RESET) },
            FrenchRank(FrenchCardRank::Four) => { concatcp!(BOLD, "4 ", RESET) },
            FrenchRank(FrenchCardRank::Five) => { concatcp!(BOLD, "5 ", RESET) },
            FrenchRank(FrenchCardRank::Six) => { concatcp!(BOLD, "6 ", RESET) },
            FrenchRank(FrenchCardRank::Seven) => { concatcp!(BOLD, "7 ", RESET) },
            FrenchRank(FrenchCardRank::Eight) => { concatcp!(BOLD, "8 ", RESET) },
            FrenchRank(FrenchCardRank::Nine) => { concatcp!(BOLD, "9 ", RESET) },
            FrenchRank(FrenchCardRank::Ten) => { concatcp!(BOLD, "10", RESET) },
            FrenchRank(FrenchCardRank::Jack) => { concatcp!(BOLD, "J ", RESET) },
            FrenchRank(FrenchCardRank::Queen) => { concatcp!(BOLD, "Q ", RESET) },
            FrenchRank(FrenchCardRank::King) => { concatcp!(BOLD, "K ", RESET) },
            FrenchRank(FrenchCardRank::Ace) => { concatcp!(BOLD, "A ", RESET) },
            GermanRank(GermanCardRank::Sieben) => { concatcp!(BOLD, "7 ", RESET) },
            GermanRank(GermanCardRank::Acht) => { concatcp!(BOLD, "8 ", RESET) },
            GermanRank(GermanCardRank::Neun) => { concatcp!(BOLD, "9 ", RESET) },
            GermanRank(GermanCardRank::Zehn) => { concatcp!(BOLD, "10", RESET) },
            GermanRank(GermanCardRank::Unter) => { concatcp!(BOLD, "U ", RESET) },
            GermanRank(GermanCardRank::Ober) => { concatcp!(BOLD, "O ", RESET) },
            GermanRank(GermanCardRank::Koing) => { concatcp!(BOLD, "K ", RESET) },
            GermanRank(GermanCardRank::Ass) => { concatcp!(BOLD, "A ", RESET) },
        }
    }
}