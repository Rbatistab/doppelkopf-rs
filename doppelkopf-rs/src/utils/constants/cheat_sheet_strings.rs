use const_format::concatcp;
use crate::utils::cli_text_style::{BOLD, RED, RESET, UNDERLINE};

pub const GAME_OVERVIEW: &str = concatcp!(
    BOLD, UNDERLINE, "\nGame Overview\n", RESET,
    "\nYou'll start with 2 decks of 24 or 20 cars ",
    "\nYou and 3 other players will be dealt 12 (or 10 cards)."
    // "You'll have to deal 2 decks (", RED, "red", RESET,
    // " and ", BOLD, "black", RESET,
    // ") of 24 (*or 20*) cards each for a total of 48 (*or 40*) cards."
);

// You'll have to deal 2 decks (red and black) of 24 (*or 20*) cards each for a total of 48 (*or 40*) cards. Remove all cards with rank lower to 9 to have 48 cards, consisting of:
// * 4 Suites (French or [German](https://en.wikipedia.org/wiki/German-suited_playing_cards) deck):
// * ♦ Diamonds (*or Bells 🇩🇪*)
// * ❤ Hearts (*german and french*)
// * ♠ Spades (*or Leaves 🇩🇪*)
// * ♣ Clubs (*or Acorns 🇩🇪*)
// * Each suit has 6 possible ranks (2 cards of each suit per rank or *'Doppelkopf'*):
// ***If you want to play with 40 cards just remove the nines.***
//
// Start by dealing 12 cards to each player (*10 for 40-card games*).The dealer is the first player to deal, and dealing passes clockwise.
