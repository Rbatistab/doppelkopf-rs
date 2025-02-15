use crate::utils::constants::trumps::{HEARTS_SOLO_TRUMPS_FRENCH, NORMAL_GAME_TRUMPS_FRENCH};
use crate::utils::text_style::{BLUE, BOLD, GREEN, ITALIC, RED, RESET, UNDERLINE};
use const_format::concatcp;
use crate::utils::constants::follow_suit::{CLUBS_FOLLOW_SUIT, SPADES_FOLLOW_SUIT};

pub const GAME_OVERVIEW: &str = concatcp!(
    BOLD, UNDERLINE, "\nGame Overview\n\n", RESET,
    BOLD, "1. Card Deal\n", RESET,
    "\n- You'll start with 2 deck of 24 or 20 cards (", RED, BOLD, "red", RESET, " and ", BOLD, "black", RESET, ").",
    "\n- One of the players will be the dealer and will deal 12 or 10 cards to each player, including himself.",
    "\n* This CLI will print ", BOLD, "black", RESET, " as ", BLUE, BOLD, "blue", RESET, " for display purposes.",
    BOLD, "\n\n2. Contract\n", RESET,
    "\n- All players say 'Fine' to agree to a normal game.",
    "\n- One or more players will say 'Hold' to ask for a different kind of game.",
    "\n- For more than one 'Hold' the highest one wins the contract: \n",
    "    ", ITALIC, "Compulsory Solo > Free Solo > Wedding", RESET,
    BOLD, "\n\n3. Play tricks and bids\n", RESET,
    "\n- The player left to the dealer plays the first trick (may make a bid too).",
    "\n- The next player to the left will play the second trick and so on until the fourth player. Optionally may bid to increase the score, but a bad bid will rest more points.",
    BOLD, "\n\n4. Evaluate the rules (scoring)\n", RESET,
    "\n- The game rules will determine the winner of the round and the amount of points that each team wins or loses.",
);

pub const CARD_DEAL: &str = concatcp!(
    BOLD, UNDERLINE, "\nCard Deal\n\n", RESET,
    "On each round, a dealer will shuffle and cut the cards. Then will deal each player with 12 cards (or 10 for 40-card game).\n",
    "Normally the dealer will yield 3 cards at the time to each player until it reaches the 12 (or 10).\n",
    "The deal for the next hand passes to the dealers left from round to round."
);

pub const CONTRACTS: &str = concatcp!(
    BOLD, UNDERLINE, "\nContracts\n\n", RESET,
    "After dealing, the players need to agree on a contract (or auction).\n",
    "All players must agree to a normal game by saying ", BOLD, "'Fine'", RESET, " ('Gesund' 🇩🇪) or one or more can call ", BOLD, "'Hold'", RESET, " ('Halt' 🇩🇪) to request for a wedding or a solo.\n\n",
    BOLD, "Always two there are, no more, no less.", RESET, " Once the contract is set, there will be two teams. The teams are either of 2 vs 2 or 1 vs 3.\n\n",
    BOLD, "1. Normal game (2 vs 2)\n\n", RESET,
    "When 2 different players have a Queens of Clubs (or 'Obers' 🇩🇪, known as 'Die Alten', the 'elders') they are a team (", BOLD, "Re", RESET, ") and will play against the other 2 (", BOLD, "Kontra", RESET, "):\n",
    "  - Re team achieves at least 121 points (or a different pre-agreed condition) -> ", BOLD, GREEN, "Re wins\n", RESET,
    "  - Re fails to get the points -> ", BOLD, GREEN, "Kontra wins\n\n", RESET,
    BOLD, "2. Wedding (2 vs 2)\n\n", RESET,
    "A player called 'suitor' who has both Queens of Clubs can call a 'wedding' and form a ", BOLD, "Re", RESET, " team with the first player to win a trick. However, if the suitor wins the first three games, he/she should play a 'Diamond solo' against the other three.\n\n",
    "A suitor may chose not to say 'wedding' on the contract and play a 'Diamond solo too', called 'Silent Solo' ('Stilles Solo' 🇩🇪).\n\n",
    BOLD, "3. Solo (1 vs 3)\n\n",
    BOLD, "May you ride eternal, shiny and chrome.", RESET, " A single player will play against the other three. He/She will earn trice the value of the score card in case of winning a trick or lose trice the value on the other case, there are:\n",
    "  - Hearts Solo\n",
    "  - Diamond Solo\n",
    "  - Jack Solo ('Bubensolo' 🇩🇪)\n",
    "  - Queen Solo ('Damensolo' 🇩🇪)\n",
    "  - Suit Solo ('Farbensolo' 🇩🇪)\n",
    "  - Ace Solo ('Fleischloser/Knochenmann' 🇩🇪)\n\n",
    "Check the 'trumps' cheat sheet for more information on trumps."
);

pub const TRICKS: &str = concatcp!(
    BOLD, UNDERLINE, "\nTricks\n\n", RESET,
    "The player left to the dealer will play the first trick which normally will by the highest trick at hand.\n",
    "Players must follow suit if possible. There are 3 options, in order of relevance to win:\n",
    "  - Play a trump, which is a card with higher rank than the others (see cheat sheet for trumps)\n",
    "  - Just follow suit: \n",
    "  - Play any card\n",
    "  - Discard\n",
    "After all tricks are played the round is won by:\n",
    "  - The highest trump\n",
    "  - The highest card of the suit led. Since each card exists twice, there is the possibility of a tie; in that case, the first-played card wins the trick\n"
);

pub const TRUMPS: &str = concatcp!(
    BOLD, UNDERLINE, "\nTrumps\n\n", RESET,
    "A trump card is a playing card that has a higher rank than other cards in a trick-taking game. The following are the trumps according to game contracts:\n\n",
    BOLD, "1. Normal Game\n\n", RESET, "Starts with the 10 of hearts, called 'Dulle' (or 'Tolle' 🇩🇪) followed by the Queen of spades, and, in decreasing order it goes like:\n\n",
    "  ", NORMAL_GAME_TRUMPS_FRENCH, "\n\n",
    "Notice most cards make a trump, the rest are non trumps but, ", BOLD, "should follow suit", RESET, ", in decreasing order:\n\n",
    "- Clubs:\n",
    "  ",CLUBS_FOLLOW_SUIT, " \n\n",
    "- Spades:\n",
    "  ", SPADES_FOLLOW_SUIT," \n\n",
    "- Hearts (10 of hears will be missing because is the highest trump):\n",
    "   \n\n",
    BOLD, "2. Wedding\n\n", RESET, "Same Trumps and suits as the normal game.\n\n",
    BOLD, "3. Solos:\n\n", RESET,
    "These are the trumps according to the type for solo game, all are in decreasing order:\n\n",
    BOLD, "  Diamond Solo:", RESET, " Same Trump as the normal game\n\n",
    BOLD, "  Hearts Solo:", RESET, " Similar to the normal game, but the 4 last trumps change.\n",
    "  These switch diamonds for hearts and remove the 10\n\n",
    "      ", HEARTS_SOLO_TRUMPS_FRENCH, "\n\n",
    "    Non-trumps:\n\n",
    "      Clubs:\n\n",
    "      ♣️ A | ♣️ 10 | ♣️ K | ♣️ 9 \n\n",
    "      Spades:\n\n",
    "      ♠️ A | ♠️ 10 | ♠️ K | ♠️ 9 \n\n",
    "      Diamonds:\n\n",
    "      ♦️ A | ♦️ 10 | ♦️ K | ♦️ 9 \n\n",
    BOLD, "  Jack Solo (Bubensolo 🇩🇪):", RESET, " Only Jacks make a Trump\n\n",
    BOLD, "  Queen Solo (Damensolo 🇩🇪):", RESET, " Only Queens make a Trump\n\n",
    BOLD, "  Suit Solo (Farbensolo 🇩🇪):", RESET, " Announce a Suit to be a Trump for Jacks and Queens\n\n",
    BOLD, "  Ace Solo (Fleischloser/Knochenmann 🇩🇪):", RESET, " There are no Trumps\n\n"
);

pub const RULES: &str = concatcp!("rules");

pub const SPECIAL_FEATURES: &str = concatcp!("special features");

pub const SCORING: &str = concatcp!("scoring");
