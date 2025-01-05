use clap::ValueEnum;

#[derive(Clone, Debug, ValueEnum)]
pub enum CheatSheetOption {
    GameOverview,
    CardDeal,
    Contracts,
    Tricks,
    Trumps,
    Rules,
    SpecialFeatures,
    Scoring
}
