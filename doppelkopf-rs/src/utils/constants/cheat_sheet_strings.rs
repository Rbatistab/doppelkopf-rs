use const_format::concatcp;
use crate::utils::cli_text_style::{BLUE, BOLD, ITALIC, RED, RESET, UNDERLINE};

pub const GAME_OVERVIEW: &str = concatcp!(
    BOLD, UNDERLINE, "\nGame Overview\n", RESET,
    BOLD, "1. Card Deal\n", RESET,
    "\n- You'll start with 2 decks of 24 or 20 cards (", RED, BOLD, "red", RESET, " and ", BOLD, "black", RESET, ").",
    "\n- One of the players will be the dealer and will deal 12 or 10 cards to each player, including himself.",
    "\n* This CLI will print ", BOLD, "black", RESET, " as ", BLUE, BOLD, "blue", RESET, " for display purposes.",
    BOLD, "\n\n2. Contract\n", RESET,
    "\n- All players say 'Fine' to agree to a normal game.",
    "\n- One or more players will say 'Hold' to ask for a different kind of game.",
    "\n- For more than one 'Hold' the highest one wins the contract: \n",
    ITALIC, "    Compulsory Solo > Free Solo > Wedding", RESET,
    BOLD, "\n\n3. Play tricks and bids\n", RESET,
    "\n- The player left to the dealer plays the first trick (may make a bid too).",
    "\n- The next player to the left will play the second trick and so on until the fourth player. Optionally may bid to increase the score, but a bad bid will rest more points.",
    BOLD, "\n\n4. Evaluate the rules (scoring)\n", RESET,
    "\n- The game rules will determine the winner of the round and the amount of points that each team wins or loses.",
);

pub const CARD_DEAL: &str = concatcp!(
    "card deal"
);

pub const CONTRACTS: &str = concatcp!(
    "contracts"
);

pub const TRICKS: &str = concatcp!(
    "tricks"
);

pub const TRUMPS: &str = concatcp!(
    "trumps"
);

pub const RULES: &str = concatcp!(
    "rules"
);

pub const SPECIAL_FEATURES: &str = concatcp!(
    "special features"
);

pub const SCORING: &str = concatcp!(
   "scoring"
);
