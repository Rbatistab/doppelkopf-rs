//! Represents card ranks, German and French
//!
//! This module contains everything related to Card ranks

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
