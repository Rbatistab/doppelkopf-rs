use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum SuitType {
    /// French suit (Diamonds, Hearts, Spades, Clubs)
    French,
    /// German suit (Bells, Hearts, Leaves, Acorns)
    German
}
