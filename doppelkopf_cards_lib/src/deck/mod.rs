//! Deck module for Doppelkopf
//!
//! Handles creation and management of deck with suit types and pack sizes
//! In this module we call 'Deck' to the merge of 2 deck, one red, one black.
//! And each of these deck can contain only Ace, 10, King, Queen, Jack, and 9

use clap::ValueEnum;
use crate::suits::SuitType;

/// Represents size of the card deck
///
/// Doppelkopf uses 2 deck (black and red) and allows for 40 cards and 48 cards variants
#[derive(Debug, PartialEq)]
pub enum PackSize {
    // Diminished pack size, Ace, 10, King, QueenJack (drops `9`s)
    Forty = 40,
    // Standard pack size (Ace, 10, King, Queen, Jack, and 9)
    FortyEight = 48
}

/// Represents a deck of cards for a Doppelkopf game
///
/// Doppelkopf uses 2 deck (black and red) and allows for 40 cards and 48 cards variants
///
/// # Fields
/// * `suit_type` - Suit (French or German)
/// * `pack_size` - Number of cards (40 or 48)
#[derive(Debug, PartialEq)]
pub struct Deck {
    suit_type: SuitType,
    pack_size: PackSize
//     cards: List<Cards>
}

impl Deck {

    /// Returns a new default deck of French Suit with 48 Cards
    /// # Returns
    /// A new Deck instance
    ///
    /// # Examples
    ///
    /// ```
    /// use doppelkopf_cards_lib::deck::{Deck, PackSize};
    /// let french_deck = Deck::default_deck();
    /// ```
    pub fn default_deck() -> Deck {
        Self {
            suit_type: SuitType::French,
            pack_size: PackSize::FortyEight
        }
    }

    /// Creates a new deck instance with the desired suit type and pack size
    /// # Returns
    /// A new Deck instance
    ///
    /// # Examples
    ///
    /// ```
    /// use doppelkopf_cards_lib::deck::{Deck, PackSize};
    /// use doppelkopf_cards_lib::suits::SuitType;
    ///
    /// let french_deck = Deck::from(SuitType::French, PackSize::FortyEight);
    /// let german_deck = Deck::from(SuitType::German, PackSize::FortyEight);
    /// ```
    pub fn from(suit_type: SuitType, pack_size: PackSize) -> Deck {
        Self {
            suit_type,
            pack_size
        }
    }
}