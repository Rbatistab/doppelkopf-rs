//! Card suit implementations for both French and German playing cards.
//!
//! This module provides representations and conversions for card suits
//! in both French (international) and German card systems.

use clap::ValueEnum;
use const_format::concatcp;
use crate::suits::Suit::{FrenchSuit, GermanSuit};
use crate::utils::constants::{BOLD, RESET};

/// Represents Suit type for the game
#[derive(Debug, PartialEq, Clone, Copy, ValueEnum)]
pub enum SuitType {
    /// French suit (Diamonds, Hearts, Spades, Clubs)
    French,
    /// German suit (Bells, Hearts, Leaves, Acorns)
    German
}


/// Represents the options for a French suit
#[derive(Debug, PartialEq)]
pub enum FrenchSuitVariant {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

/// Represents the options for a German suit
#[derive(Debug, PartialEq)]
pub enum GermanSuitVariant {
    Eichel, // Acorns
    Schell, // Bells
    Herz,   // Hearts
    Grun,   // Leaves
}

/// Represents the suit of a card, for either German or French cards
#[derive(Debug, PartialEq)]
pub enum Suit {
    FrenchSuit(FrenchSuitVariant),
    GermanSuit(GermanSuitVariant),
}

impl Suit {
    /// Converts a suit to its string representation.
    ///
    /// Returns a static string with a consistent 2-character width
    /// (padded with space where needed).
    ///
    /// # Examples
    ///
    /// This example uses `BOLD` and `RESET` formating because the suits are bolded
    ///
    /// ```
    /// use std::fmt::format;
    /// use doppelkopf_cards_lib::suits::Suit;
    /// use doppelkopf_cards_lib::suits::FrenchSuitVariant::Spades;
    /// use doppelkopf_cards_lib::suits::GermanSuitVariant::Schell;
    ///
    /// const BOLD: &str = "\x1b[1m";
    /// const RESET: &str = "\x1b[0m";
    /// let bold_spade = format!("{}♠{}", BOLD, RESET);
    ///
    /// let french_rank = Suit::FrenchSuit(Spades).to_str();
    /// let german_rank = Suit::GermanSuit(Schell).to_str();
    ///
    /// assert_eq!(french_rank, bold_spade);
    /// assert_eq!(german_rank, "🔔");
    /// ```
    pub fn to_str(&self) -> &'static str {
        match self {
            FrenchSuit(FrenchSuitVariant::Clubs) => { concatcp!(BOLD, "♣ ", RESET)},
            FrenchSuit(FrenchSuitVariant::Diamonds) => { concatcp!(BOLD, "♦ ", RESET)},
            FrenchSuit(FrenchSuitVariant::Hearts) => { concatcp!(BOLD, "♥ ", RESET)},
            FrenchSuit(FrenchSuitVariant::Spades) => { concatcp!(BOLD, "♠ ", RESET)},
            GermanSuit(GermanSuitVariant::Eichel) => { "🌰" },
            GermanSuit(GermanSuitVariant::Schell) => { "🔔" },
            GermanSuit(GermanSuitVariant::Herz) => { "💙" },
            GermanSuit(GermanSuitVariant::Grun) => { "🍃" },
        }
    }
}