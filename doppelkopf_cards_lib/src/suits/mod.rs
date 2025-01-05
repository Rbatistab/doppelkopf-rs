use clap::ValueEnum;

/// Represents Suit type for the game
#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum SuitType {
    /// French suit (Diamonds, Hearts, Spades, Clubs)
    French,
    /// German suit (Bells, Hearts, Leaves, Acorns)
    German
}
