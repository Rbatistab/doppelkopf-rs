//! [Card] color handling for playing cards
//!
//! This module provides color representation for playing cards, specifically
//! handling the distinction between red and black cards. It includes ANSI
//! terminal color support for visual representation.


use crate::utils::constants::RED;

/// Represents card color
#[derive(Debug, PartialEq)]
pub enum CardColor {
    BLACK,
    RED
}

impl CardColor {
    /// Converts the card color to its ANSI terminal color code representation.
    ///
    /// # Returns
    /// - For RED: Returns the ANSI escape sequence for red color
    /// - For BLACK: Returns an empty string, using the terminal's default color
    ///
    /// # Note
    /// Black cards use the default terminal color (empty string) to maintain
    /// better compatibility with different terminal color schemes.
    pub fn to_str(&self) -> &'static str {
       match self {
           // Keep black as "" since we don't want to format tha black color
           // We want the default color in the terminal to be the black card
           CardColor::BLACK => { "" },
           CardColor::RED => { RED },
       }
    }
}

#[cfg(test)]
mod card_color_tests {
    use super::*;

    #[test]
    fn test_to_str() {
        let red = CardColor::RED;
        let black = CardColor::BLACK;

        assert_eq!(red.to_str(), "\x1b[31m");
        assert_eq!(black.to_str(), "");
    }
}