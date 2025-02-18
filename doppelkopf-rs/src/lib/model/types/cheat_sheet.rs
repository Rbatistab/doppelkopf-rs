//! Represents cheat sheet options that a user can request.

use clap::ValueEnum;

/// Represent the desired cheat sheet.
#[derive(Clone, Debug, PartialEq, ValueEnum)]
pub enum CheatSheetOption {
    // Very general game overview
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
