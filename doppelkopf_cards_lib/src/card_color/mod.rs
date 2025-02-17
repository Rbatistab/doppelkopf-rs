//! [Card] color handling for playing cards
//!
//! This module provides color representation for playing cards, specifically
//! handling the distinction between red and black cards. It includes ANSI
//! terminal color support for visual representation.


use crate::utils::constants::{BLUE, RED};

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
           // Keep black as blue since we don't want to format tha black color
           // A user's terminal may have a light or dark theme
           // We want the default black color in the terminal to be the blue
           CardColor::BLACK => { BLUE },
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
        assert_eq!(black.to_str(), "\x1b[34m");
    }
}