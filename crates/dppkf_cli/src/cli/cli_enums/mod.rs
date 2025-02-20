use clap::ValueEnum;

/// Represents Suit type for the game. Duplication of dppkf_cards_lib enum for CLI option purpose
#[derive(Debug, Clone, ValueEnum)]
pub enum SuitTypeCli {
    /// French suit (Diamonds, Hearts, Spades, Clubs)
    French,
    /// German suit (Bells, Hearts, Leaves, Acorns)
    German
}

/// Represent the desired cheat sheet.Duplication of dppkf_lib enum for CLI option purpose
#[derive(Debug, Clone, ValueEnum)]
pub enum CheatSheetOptionCli {
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
