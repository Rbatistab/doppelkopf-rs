//! German and French cards struct and builder functions
//!
//! # Card Struct
//!
//! In order to play Doppelkopf we need cards which may be either French or German suited which we
//! represent with a struct, `Card`
//!
//! ## Example:
//! ```
//! use doppelkopf_cards_lib::card::Card;
//! use doppelkopf_cards_lib::ranks::Rank;
//! use doppelkopf_cards_lib::suits::{Suit, SuitType};
//! use doppelkopf_cards_lib::ranks::FrenchCardRank::Ace;
//! use doppelkopf_cards_lib::suits::FrenchSuitVariant::Spades;
//!
//! let manual_ace_of_spades = Card {
//!     rank: Rank::FrenchRank(Ace),
//!     suit: Suit::FrenchSuit(Spades),
//!     suit_type: SuitType::French
//! };
//!
//! println!("This is a manually created ace of spades: {:?}", manual_ace_of_spades);
//! ```
//!
//!
//! # Card::new("Rank-Suit") syntax
//!
//! Cards struct offer an implementation with the `new` associated function to build a new card from
//! an `&str` representation like this: `Rank-Suit`
//!
//! We'll start from the following 2 regexs:
//! French regex: `\b([2-9]|10|J|Q|K|A)-(C|D|H|S)\b`
//! German regex: `\b([7-9]|10|U|O|K|A)-(E|Sc|He|G)\b`
//! - Rank part:
//!   - `2-10|J|Q|K|A`: French ranks 2 to 10, Jack, Queen, King, Ace
//!   - `7-10|U|O|K|A`: German ranks 7 to 10, Unter, Ober, Koing, Ass
//! - Suit part:
//!   - `C|D|H|S`: French Clubs, Diamonds, Hearts, Spades
//!   - `E|Sc|He|G`: German Eichel(acorn), Schell(bells), herz(hearts) and Grun(leaves)
//!
//! ## French suited syntax:
//! | Rank | Rank str | Suit     | Suit str |
//! |------|----------|----------|----------|
//! | 2    | 2        | Clubs    | C        |
//! | 3    | 3        | Diamonds | D        |
//! | 4    | 4        | Hearts   | H        |
//! | 5    | 5        | Spades   | S        |
//! | 6    | 6        |-         |-         |
//! | 7    | 7        |-         |-         |
//! | 8    | 8        |-         |-         |
//! | 8    | 8        |-         |-         |
//! | 9    | 9        |-         |-         |
//! | 10   | 10       |-         |-         |
//! | Jack | J        |-         |-         |
//! | Queen| Q        |-         |-         |
//! | King | K        |-         |-         |
//! | Ace  | A        |-         |-         |
//!
//! Some examples:
//!  * Queen of Hearts: `Q-H`
//!  * Seven of Spades: `7-S`
//!
//! ## German suited syntax:
//!
//! | Rank | Rank str | Suit         | Suit str |
//! |------|----------|--------------|----------|
//! | 7    | 7        |Eichel(Acorns)|E         |
//! | 8    | 8        |Schell(Bells) |Sc        |
//! | 8    | 8        |Herz(Hearts)  |He        |
//! | 9    | 9        |Grun(Leaves)  |G         |
//! | 10   | 10       |-             |-         |
//! | Unter| U        |-             |-         |
//! | Ober | O        |-             |-         |
//! | Koing| K        |-             |-         |
//! | Ass  | A        |-             |-         |
//!
//! Some examples:
//!  * Queen(Ober) of Hearts: `O-H`
//!  * "The fox", Ace of Bells: `A-Sc`
//!
//! # Example:
//!
//! ```
//! use doppelkopf_cards_lib::card::Card;
//! use doppelkopf_cards_lib::ranks::Rank;
//! use doppelkopf_cards_lib::suits::{Suit, SuitType};
//! use doppelkopf_cards_lib::ranks::FrenchCardRank::Ace;
//! use doppelkopf_cards_lib::ranks::GermanCardRank::Ass;
//! use doppelkopf_cards_lib::suits::FrenchSuitVariant::Spades;
//! use doppelkopf_cards_lib::suits::GermanSuitVariant::Schell;
//!
//! // Let's start with a French suited card
//! let ace_of_spaces  = Card::new("A-S");
//! println!("This is a French suited ace of spades from the new associated function: {:?}", ace_of_spaces);
//! assert_eq!(ace_of_spaces.rank, Rank::FrenchRank(Ace));
//! assert_eq!(ace_of_spaces.suit, Suit::FrenchSuit(Spades));
//! assert_eq!(ace_of_spaces.suit_type, SuitType::French);
//!
//! // Now let's make a German suited card:
//! let fox = Card::new("A-Sc");
//! println!("This is a German suited Ace of Bells from the new associated function: {:?}", fox);
//! assert_eq!(fox.rank, Rank::GermanRank(Ass));
//! assert_eq!(fox.suit, Suit::GermanSuit(Schell));
//! assert_eq!(fox.suit_type, SuitType::German);
//! ```

use regex::Regex;

use crate::ranks::FrenchCardRank::{Ace, Eight, Five, Four, Jack, King, Nine, Queen, Seven, Six, Ten, Three, Two};
use crate::ranks::GermanCardRank::{Acht, Ass, Koing, Neun, Ober, Sieben, Unter, Zehn};
use crate::ranks::Rank;
use crate::suits::FrenchSuitVariant::{Clubs, Diamonds, Hearts, Spades};
use crate::suits::GermanSuitVariant::{Eichel, Schell, Herz, Grun};
use crate::suits::{Suit, SuitType};

const FRENCH_SUIT_REGEX: &str = r"^\b([2-9]|10|J|Q|K|A)-(C|D|H|S)\b$";
const GERMAN_SUIT_REGEX: &str = r"^\b([7-9]|10|U|O|K|A)-(E|Sc|He|G)\b$";

/// Represents a card that has a rank, a suit and a suit type (French/German)
///
/// # Fields
///
/// * `rank` -  Rank of the card
/// * `suit` - Suit of the card
/// * `suit_type` - Type of suit (German or French)
///
/// # Examples
/// ```
/// use doppelkopf_cards_lib::card::Card;
/// use doppelkopf_cards_lib::ranks::FrenchCardRank::Ace;
/// use doppelkopf_cards_lib::ranks::Rank;
/// use doppelkopf_cards_lib::suits::FrenchSuitVariant::Spades;
/// use doppelkopf_cards_lib::suits::{Suit, SuitType};
///
/// let ace_of_spades = Card {
///     rank: Rank::FrenchRank(Ace),
///     suit: Suit::FrenchSuit(Spades),
///     suit_type: SuitType::French
/// };
/// ```
#[derive(Debug, PartialEq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub suit_type: SuitType,
}

impl Card {
    /// Creates a new card from a str
    ///
    /// # Returns Card instance
    ///
    /// # Fields
    /// `card_string` - String to represent the card, as `Rank-Suit` which should match one of 2 regex
    ///
    /// # String representation: Rank-Suit
    /// French regex: `\b([2-9]|10|J|Q|K|A)-(C|D|H|S)\b`
    /// German regex: `\b([7-9]|10|U|O|K|A)-(E|Sc|He|G)\b`
    /// - Rank part:
    ///   - `2-10|J|Q|K|A`: French ranks 2 to 10, Jack, Queen, King, Ace
    ///   - `7-10|U|O|K|A`: German ranks 7 to 10, Unter, Ober, Koing, Ass
    /// - Suit part:
    ///   - `C|D|H|S`: French Clubs, Diamonds, Hearts, Spades
    ///   - `E|Sc|He|G`: German Eichel(acorn), Schell(bells), herz(hearts) and Grun(leaves)
    ///
    /// # Examples
    /// ```
    /// use doppelkopf_cards_lib::card::Card;
    /// use doppelkopf_cards_lib::ranks::FrenchCardRank::Ace;
    /// use doppelkopf_cards_lib::ranks::Rank;
    /// use doppelkopf_cards_lib::suits::FrenchSuitVariant::Spades;
    /// use doppelkopf_cards_lib::suits::{Suit, SuitType};
    ///
    /// let ace_of_spaces  = Card::new("A-S");
    ///
    /// assert_eq!(ace_of_spaces.rank, Rank::FrenchRank(Ace));
    /// assert_eq!(ace_of_spaces.suit, Suit::FrenchSuit(Spades));
    /// assert_eq!(ace_of_spaces.suit_type, SuitType::French);
    /// ```
    pub fn new(card_str: &str) -> Card {
        match Self::get_suit_type(card_str) {
            Some(SuitType::French) => {
                Self::get_french_card_from_str(card_str)
            },
            Some(SuitType::German) => {
                Self::get_german_card_from_str(card_str)
            },
            None => {
                eprintln!("Invalid regex");
                panic!("Failed to create new card from {}", card_str);
            }
        }
    }

    /// Validates the str representing the card against two regexes to determine the suit type and returns accordingly
    /// French regex: `\b(10|[2-9]|J|Q|K|A)-(C|D|H|S)\b`
    /// German regex: `\b(10|[7-9]|U|O|Ko|As)-(A|B|He|L)\b`
    ///
    /// This is used internally to prevent invalid card creation.
    fn get_suit_type(card_str: &str) -> Option<SuitType> {
        let french_regex = Regex::new(FRENCH_SUIT_REGEX).unwrap();
        let german_regex = Regex::new(GERMAN_SUIT_REGEX).unwrap();

        if french_regex.is_match(card_str) {
            Some(SuitType::French)
        } else if german_regex.is_match(card_str) {
            Some(SuitType::German)
        } else {
            None
        }
    }

    /// Gets a French Card instance from the coded str
    ///
    /// This is used internally to determine the suit type
    fn get_french_card_from_str(card_str: &str) -> Card {
        let rank: Rank;
        let suit: Suit;
        if let Some((rank_str, suit_str)) = card_str.split_once('-') {

            if suit_str == "C" {
                suit = Suit::FrenchSuit(Clubs);
            } else if suit_str == "D" {
                suit = Suit::FrenchSuit(Diamonds);
            } else if suit_str == "H" {
                suit = Suit::FrenchSuit(Hearts);
            } else if suit_str == "S" {
                suit = Suit::FrenchSuit(Spades);
            } else {
                eprintln!("Invalid suit");
                panic!("Failed to create new card with suit coded as '{}'", suit_str);
            }

            if rank_str == "2" {
                rank = Rank::FrenchRank(Two);
            } else if rank_str == "3" {
                rank = Rank::FrenchRank(Three);
            } else if rank_str == "4" {
                rank = Rank::FrenchRank(Four);
            } else if rank_str == "5" {
                rank = Rank::FrenchRank(Five);
            } else if rank_str == "6" {
                rank = Rank::FrenchRank(Six);
            } else if rank_str == "7" {
                rank = Rank::FrenchRank(Seven);
            } else if rank_str == "8" {
                rank = Rank::FrenchRank(Eight);
            } else if rank_str == "9" {
                rank = Rank::FrenchRank(Nine);
            } else if rank_str == "10" {
                rank = Rank::FrenchRank(Ten);
            } else if rank_str == "J" {
                rank = Rank::FrenchRank(Jack);
            } else if rank_str == "Q" {
                rank = Rank::FrenchRank(Queen);
            } else if rank_str == "K" {
                rank = Rank::FrenchRank(King);
            } else if rank_str == "A" {
                rank = Rank::FrenchRank(Ace);
            } else {
                eprintln!("Invalid rank");
                panic!("Failed to create new card with rank coded as '{}'", rank_str);
            }

            Card {
                rank,
                suit,
                suit_type: SuitType::French
            }

        } else {
            eprintln!("The rank and the suit should be split by a '-' character. Regex should prevent this situation.");
            panic!("Couldn't split the rank and the suit of {}", card_str);
        }

    }

    /// Gets a German Card instance from the coded str
    ///
    /// This is used internally to determine the suit and rank
    fn get_german_card_from_str(card_str: &str) -> Card {
        let rank: Rank;
        let suit: Suit;
        if let Some((rank_str, suit_str)) = card_str.split_once('-') {

            if suit_str == "E" {
                suit = Suit::GermanSuit(Eichel);
            } else if suit_str == "Sc" {
                suit = Suit::GermanSuit(Schell);
            } else if suit_str == "He" {
                suit = Suit::GermanSuit(Herz);
            } else if suit_str == "G" {
                suit = Suit::GermanSuit(Grun);
            } else {
                eprintln!("Invalid suit");
                panic!("Failed to create new card with suit coded as '{}'", suit_str);
            }

            if rank_str == "7" {
                rank = Rank::GermanRank(Sieben);
            } else if rank_str == "8" {
                rank = Rank::GermanRank(Acht);
            } else if rank_str == "9" {
                rank = Rank::GermanRank(Neun);
            } else if rank_str == "10" {
                rank = Rank::GermanRank(Zehn);
            } else if rank_str == "U" {
                rank = Rank::GermanRank(Unter);
            } else if rank_str == "O" {
                rank = Rank::GermanRank(Ober);
            } else if rank_str == "K" {
                rank = Rank::GermanRank(Koing);
            } else if rank_str == "A" {
                rank = Rank::GermanRank(Ass);
            } else {
                eprintln!("Invalid rank");
                panic!("Failed to create new card with rank coded as '{}'", rank_str);
            }

            Card {
                rank,
                suit,
                suit_type: SuitType::German
            }

        } else {
            eprintln!("The rank and the suit should be split by a '-' character. Regex should prevent this situation.");
            panic!("Couldn't split the rank and the suit of {}", card_str);
        }
    }

}

#[cfg(test)]
mod card_tests {
    use super::*;

    #[test]
    fn test_validate_card_regex_helper() {
        let valid_french_regex: &str = "10-C";
        let invalid_french_regex: &str = "11-G";
        let valid_german_regex: &str = "7-G";
        let invalid_german_regex: &str = "1-G";
        let invalid_regex: &str = "0-P";

        assert_eq!(Card::get_suit_type(valid_french_regex), Some(SuitType::French));
        assert_eq!(Card::get_suit_type(invalid_french_regex), None);
        assert_eq!(Card::get_suit_type(valid_german_regex), Some(SuitType::German));
        assert_eq!(Card::get_suit_type(invalid_german_regex), None);
        assert_eq!(Card::get_suit_type(invalid_regex), None);
    }

    #[test]
    fn test_get_french_card_from_str() {
        let valid_french_regex: &str = "10-C";

        let valid_french_card = Card {
            rank: Rank::FrenchRank(Ten),
            suit: Suit::FrenchSuit(Clubs),
            suit_type: SuitType::French
        };

        assert_eq!(Card::get_french_card_from_str(valid_french_regex), valid_french_card);
    }

    #[test]
    #[should_panic]
    fn test_get_french_card_from_str_panic() {
        let invalid_french_regex: &str = "2-Sc";
        let _invalid_card = Card::get_french_card_from_str(invalid_french_regex);
    }

    #[test]
    fn test_get_german_card_from_str() {
        let valid_german_regex: &str = "7-Sc";

        let valid_german_card = Card {
            rank: Rank::GermanRank(Sieben),
            suit: Suit::GermanSuit(Schell),
            suit_type: SuitType::German
        };

        assert_eq!(Card::get_german_card_from_str(valid_german_regex), valid_german_card);
    }

    #[test]
    #[should_panic]
    fn test_get_german_card_from_str_panic() {
        let invalid_german_regex: &str = "2-A";
        let _invalid_card = Card::get_german_card_from_str(invalid_german_regex);
    }

}