//! Represents cheat sheet options that a user can request.
//!
//! This module provides an enumeration of different help topics available
//! in the Doppelkopf game, allowing players to access specific game rules
//! and mechanics information.

/// Represents the different types of help topics available in the game.
///
/// Each variant corresponds to a specific aspect of the Doppelkopf game,
/// from basic game overview to detailed scoring rules.
#[derive(Clone, Debug, PartialEq)]
pub enum CheatSheetOption {
    /// Provides a general overview of how Doppelkopf is played
    GameOverview,
    // How card dealing works in the game
    CardDeal,
    // How to play bids (or contracts) on a game
    Contracts,
    // How to play tricks
    Tricks,
    // Announcements to multiply the score
    Bids,
    // Which are the trumps to win a round
    Trumps,
    // The game rules
    Rules,
    // Special features to increase/decrease scoring
    SpecialFeatures,
    // How scoring works
    Scoring
}

impl CheatSheetOption {
    /// Converts the enum variant to its corresponding string representation.
    ///
    /// Returns a static string that describes the selected help topic.
    ///
    /// # Returns
    ///
    /// A `&'static str` containing the human-readable description of the help topic.
    ///
    /// # Examples
    ///
    /// ```
    /// use dppkf_lib::model::types::cheat_sheet::CheatSheetOption;
    ///
    /// let option = CheatSheetOption::GameOverview;
    /// assert_eq!(option.to_str(), "Game overview");
    /// ```
    pub fn to_str(&self) -> &'static str {
        match self {
            CheatSheetOption::GameOverview => "Game overview",
            CheatSheetOption::CardDeal => "Card deal",
            CheatSheetOption::Contracts => "Contracts",
            CheatSheetOption::Tricks => "Tricks",
            CheatSheetOption::Bids => "Bids",
            CheatSheetOption::Trumps => "Trumps",
            CheatSheetOption::Rules => "Rules",
            CheatSheetOption::SpecialFeatures => "Special features",
            CheatSheetOption::Scoring => "Scoring",
        }
    }
}

impl std::fmt::Display for CheatSheetOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
