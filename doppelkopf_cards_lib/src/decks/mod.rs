use clap::ValueEnum;
use crate::suits::SuitType;

// #[derive(Debug, Clone, ValueEnum)]
// pub enum PackSize {
//     FORTY = 40,
//     FORTY_EIGHT = 48
// }

#[derive(Debug)]
pub struct Deck {
    suit_type: SuitType
}

impl Deck {
    pub fn from(suit_type: SuitType) -> Deck {
        Self {
           suit_type
        }
    }
}