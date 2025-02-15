//! Represents card suits, German and French
//!
//! This module contains everything related to Card suits

use clap::ValueEnum;

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