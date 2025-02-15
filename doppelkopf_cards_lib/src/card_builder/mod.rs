//! Builder design pattern for Card instances
//!
//! # CardBuilder
//!
//! This is a straightforward builder pattern for a Card instance. Notice it won't consume a SuitType argument, since it takes it from the rank or the suit and validates it's correctness.
//!
//! ## Example
//! ```
//! use doppelkopf_cards_lib::card::Card;
//! use doppelkopf_cards_lib::card_builder::CardBuilder;
//! use doppelkopf_cards_lib::ranks::FrenchCardRank::Ace;
//! use doppelkopf_cards_lib::ranks::Rank;
//! use doppelkopf_cards_lib::suits::FrenchSuitVariant::Spades;
//! use doppelkopf_cards_lib::suits::{Suit, SuitType};
//!
//! let ace_of_spades = CardBuilder::new()
//!     .with_rank(Rank::FrenchRank(Ace))
//!     .with_suit(Suit::FrenchSuit(Spades))
//!     .build();
//!
//! assert_eq!(ace_of_spades.rank, Rank::FrenchRank(Ace));
//! assert_eq!(ace_of_spades.suit, Suit::FrenchSuit(Spades));
//! assert_eq!(ace_of_spades.suit_type, SuitType::French);
//! ```

use crate::card::Card;
use crate::ranks::Rank;
use crate::suits::{Suit, SuitType};

/// Builder struct for Card instance
///
/// # Fields
///
/// * `rank` -  Option for Rank instance
/// * `suit` - Option for Suit instance
/// * `suit_type` - Option for SuitType instance
#[derive(Debug, PartialEq)]
pub struct CardBuilder {
    rank: Option<Rank>,
    suit: Option<Suit>,
    suit_type: Option<SuitType>
}

impl CardBuilder {

    /// Builder pattern for Card instance
    ///
    /// # Creates a new CardBuilder instance
    ///
    /// # Methods:
    ///
    /// - `with_rank` - Sets the Rank
    /// - `with_suit` - Sets the Suit
    /// - `with_suit_type` - Sets the SuitType
    ///
    /// # Panics
    /// Will panic if the card already has a suit type, but you try to assign a rank from a different one
    ///
    /// # Examples
    pub fn new() -> Self {
        CardBuilder {
            rank: None,
            suit: None,
            suit_type: None
        }
    }

    /// Builder pattern for Card instance
    ///
    /// # Sets a rank for a new card builder
    ///
    /// # Panics
    /// Will panic if the card already has a suit type, and you assign a rank from a different one
    pub fn with_rank(mut self, rank: Rank) -> CardBuilder {
        let rank_suit_type = Self::get_suit_type_from_rank(&rank);

        match self.suit_type {
            Some(SuitType::French) | Some(SuitType::German) => {
                if self.suit_type == Some(rank_suit_type) {
                    self.rank = Some(rank);
                    self
                } else {
                    eprintln!("Can't mix two different suit types, your rank and suit must be of the same type of card.");
                    panic!("Can't assign {:?}, with a suit type {:?}, to a card of {:?} suit type.", rank, rank_suit_type, self.suit_type.unwrap());
                }
            },
            None => {
                self.suit_type = Some(rank_suit_type);
                self.rank = Some(rank);
                self
            }
        }
    }

    /// Builder pattern for Card instance
    ///
    /// # Sets a rank for a new card builder
    ///
    /// # Panics
    /// Will panic if the card already has a suit type, and you assign a suit from a different one
    pub fn with_suit(mut self, suit: Suit) -> CardBuilder {
        let suit_suit_type = CardBuilder::get_suit_type_from_suit(&suit);

        match self.suit_type {
            Some(SuitType::French) | Some(SuitType::German) => {
               if self.suit_type == Some(suit_suit_type) {
                   self.suit = Some(suit);
                   self
               } else {
                   eprintln!("Can't mix two different suit types, your rank and suit must be of the same type of card.");
                   panic!("Can't assign {:?}, with a suit type {:?}, to a card of {:?} suit type.", suit, suit_suit_type, self.suit_type.unwrap());
               }
            },
            None => {
                self.suit_type = Some(suit_suit_type);
                self.suit = Some(suit);
                self
            }
        }
    }

    /// Builds a Card instance
    ///
    /// # Panics
    /// If any of the Card fields are missing
    pub fn build(self) -> Card {
        // Should validate it has all fields!!
        Card {
            rank: self.rank.expect("Can't build a Card if the rank is not defined."),
            suit: self.suit.expect("Can't build a Card if the suit is node defined."),
            suit_type: self.suit_type.expect("Can't build a Card if the suit type is not defined.")
        }
    }

    /// Gets suit type from a Rank variant
    ///
    /// This is used as an internal util
    fn get_suit_type_from_rank(rank: &Rank) -> SuitType {
        match rank {
            Rank::FrenchRank(_) => SuitType::French,
            Rank::GermanRank(_) => SuitType::German,
        }
    }

    /// Gets suit type from a Suit variant
    ///
    /// This is used as an internal util
    fn get_suit_type_from_suit(suit: &Suit) -> SuitType {
        match suit {
            Suit::FrenchSuit(_) => SuitType::French,
            Suit::GermanSuit(_) => SuitType::German,
        }
    }

}

impl Default for CardBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod card_builder_tests {
    use crate::ranks::FrenchCardRank::Two;
    use crate::ranks::GermanCardRank::Sieben;
    use crate::ranks::Rank;
    use crate::suits::FrenchSuitVariant::{Diamonds, Hearts};
    use crate::suits::GermanSuitVariant::{Grun, Herz};
    use super::*;

    #[test]
    fn test_with_rank_new_builder() {
        let french_rank = Rank::FrenchRank(Two);
        let german_rank = Rank::GermanRank(Sieben);
        let french_builder = CardBuilder::new()
            .with_rank(french_rank);
        let german_builder = CardBuilder::new()
            .with_rank(german_rank);

        assert_eq!(french_builder.rank, Some(Rank::FrenchRank(Two)));
        assert_eq!(german_builder.rank, Some(Rank::GermanRank(Sieben)));
    }

    #[test]
    #[should_panic]
    fn test_with_rank_mixed_suit_type() {
        let german_rank = Rank::GermanRank(Sieben);
        let french_builder = CardBuilder {
            rank: None,
            suit: None,
            suit_type: Some(SuitType::French)
        };

        french_builder.with_rank(german_rank);
    }

    #[test]
    fn test_with_suit_new_builder() {
        let french_suit= Suit::FrenchSuit(Hearts);
        let german_suit  = Suit::GermanSuit(Herz);
        let french_builder = CardBuilder::new()
            .with_suit(french_suit);
        let german_builder = CardBuilder::new()
            .with_suit(german_suit);

        assert_eq!(french_builder.suit, Some(Suit::FrenchSuit(Hearts)));
        assert_eq!(german_builder.suit, Some(Suit::GermanSuit(Herz)));
    }

    #[test]
    #[should_panic]
    fn test_with_suit_mixed_suit_type() {
        let french_suit = Suit::FrenchSuit(Diamonds);
        let german_builder = CardBuilder {
            rank: None,
            suit: None,
            suit_type: Some(SuitType::German)
        };

        german_builder.with_suit(french_suit);
    }

    #[test]
    fn test_get_suit_type_from_rank() {
        let french_rank = Rank::FrenchRank(Two);
        let german_rank = Rank::GermanRank(Sieben);

        let expected_french = CardBuilder::get_suit_type_from_rank(&french_rank);
        let expected_german = CardBuilder::get_suit_type_from_rank(&german_rank);

        assert_eq!(expected_french, SuitType::French);
        assert_eq!(expected_german, SuitType::German);
    }

    #[test]
    fn test_get_suit_type_from_suit() {
        let french_suit = Suit::FrenchSuit(Hearts);
        let german_suit = Suit::GermanSuit(Grun);

        let expected_french = CardBuilder::get_suit_type_from_suit(&french_suit);
        let expected_german = CardBuilder::get_suit_type_from_suit(&german_suit);

        assert_eq!(expected_french, SuitType::French);
        assert_eq!(expected_german, SuitType::German);
    }

    #[test]
    fn test_build_valid_case() {
        let french_rank = Rank::FrenchRank(Two);
        let french_suit = Suit::FrenchSuit(Hearts);

        let card = CardBuilder::new()
            .with_rank(french_rank)
            .with_suit(french_suit)
            .build();

        assert_eq!(card.rank, Rank::FrenchRank(Two));
        assert_eq!(card.suit, Suit::FrenchSuit(Hearts));
        assert_eq!(card.suit_type, SuitType::French);
    }

    #[test]
    #[should_panic]
    fn test_build_missing_fields() {
        let french_rank = Rank::FrenchRank(Two);

        let card = CardBuilder::new()
            .with_rank(french_rank)
            .build();
    }
}