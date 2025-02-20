use crate::utils::constants::follow_suit::{CLUBS_FOLLOW_SUIT, SPADES_FOLLOW_SUIT};
use crate::utils::constants::trumps::{HEARTS_SOLO_TRUMPS_FRENCH, NORMAL_GAME_TRUMPS_FRENCH};
use crate::utils::text_style::{BLUE, BOLD, GREEN, ITALIC, RED, RESET, UNDERLINE};
use const_format::concatcp;

pub const GAME_OVERVIEW: &str = concatcp!(
    BOLD, UNDERLINE, "\nGame Overview\n\n", RESET,
    BOLD, "1. Card Deal\n", RESET,
    "\n- You'll start with 2 deck of 24 or 20 cards (", RED, BOLD, "red", RESET, " and ", BOLD, "black", RESET, ").",
    "\n- One of the players will be the dealer and will deal 12 or 10 cards to each player, including himself.",
    "\n\n* This CLI will print ", BOLD, "black", RESET, " as ", BLUE, BOLD, "blue", RESET, " for display purposes.",
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
    "Normally the dealer will yield 3 cards at the time to each player until it reaches the 12 (or 10).\n\n",
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
    "  - Just follow suit \n",
    "  - Play any card\n",
    "  - Discard\n\n",
    "After all tricks are played the round is won by:\n",
    "  - The highest trump\n",
    "  - The highest card of the suit led. Since each card exists twice, there is the possibility of a tie; in that case, the first-played card wins the trick\n"
);

pub const BIDS: &str = concatcp!(
    BOLD, UNDERLINE, "\nBids\n\n", RESET,
    "A player may make announcements claiming that their team will succeed in achieving a specific goal.\n",
    "These announcements increase the game value regardless of whether they are fulfilled. However, when a team fails a self-given goal, they automatically lose.\n\n",
    "The bids, in order, are:\n",
    " - ", BOLD, BLUE, "Re/Contra", RESET, ": The name of the player's team, claims that the team will make more than 120 points. and undertaking that their team will score more than 120 points. All following bids should follow this bid, this is the first valid bid => Doubles the value\n",
    "    > Can only be played with 11 cards left\n",
    " - ", BOLD, BLUE, "No 90", RESET, " (also called '", BLUE, "No 9", RESET, "'): Claims the opponents will get less than 90 points => Triples the value\n",
    "    > Can only be played with 10 cards left\n",
    " - ", BOLD, BLUE, "No 60", RESET, " (also called '", BLUE, "No 6", RESET, "'): Claims the opposing team will not make 60 points => X4 the value\n",
    "    > Can only be played with 9 cards left\n",
    " - ", BOLD, BLUE, "No 30", RESET, " (also called '", BLUE, "No 3", RESET, "': You get the idea\n",
    "    > Can only be played with 8 cards left\n",
    " - ", BOLD, BLUE, "Schwarz", RESET, ": Claims the opponents will not get a single trick, not even a trick worth zero points\n",
    "    > Can only be played with 7 cards left\n\n",
    "The fun part here is that each bid will imply all previous: 'No 6' will imply 'No 9' which will imply 'Re/Contra', hence the game increases by 4.\n",
    "Also, every bid can have a counterbid making both valid."
);

pub const TRUMPS: &str = concatcp!(
    BOLD, UNDERLINE, "\nTrumps\n\n", RESET,
    "A trump card is a playing card that has a higher rank than other cards in a trick-taking game. The following are the trumps according to game contracts:\n\n",
    BOLD, "1. Normal Game\n\n", RESET,
    "Starts with the 10 of hearts, called 'Dulle' (or 'Tolle' 🇩🇪) followed by the Queen of spades, and, in decreasing order it goes like:\n\n",
    "  ", NORMAL_GAME_TRUMPS_FRENCH, "\n\n",
    "Notice most cards make a trump, the rest are non trumps but, ", BOLD, "should follow suit", RESET, ", in decreasing order:\n\n",
    "- Clubs:\n",
    "  ",CLUBS_FOLLOW_SUIT, " \n\n",
    "- Spades:\n",
    "  ", SPADES_FOLLOW_SUIT," \n\n",
    "- Hearts (10 of hears will be missing because is the highest trump):\n",
    "  ❤️ A | ❤️ K | ❤️ 9 \n\n",
    BOLD, "2. Wedding\n\n", RESET,
    "Same Trumps and suits as the normal game.\n\n",
    BOLD, "3. Solos:\n\n", RESET,
    "These are the trumps according to the type for solo game, all are in decreasing order:\n\n",
    BOLD, "  Diamond Solo:", RESET, " Same Trump as the normal game.\n\n",
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
    BOLD, "  Jack Solo ('Bubensolo' 🇩🇪):", RESET, " Only Jacks make a Trump\n\n",
    "      ♣️ J | ♠️ J | ❤️ J | ♦️ J \n\n",
    BOLD, "  Queen Solo ('Damensolo' 🇩🇪):", RESET, " Only Queens make a Trump\n\n",
    "      ♣️ Q | ♠️ Q | ❤️ Q | ♦️ Q \n\n",
    BOLD, "  Ace Solo ('Fleischloser/Knochenmann' 🇩🇪):", RESET, " There are no Trumps\n\n",
    "      ♣️ A | ♠️ A | ❤️ A | ♦️ A \n\n"
);

pub const RULES: &str = concatcp!(
    BOLD, UNDERLINE, "\nRules\n\n", RESET,
    "1. Basic setup: \n\n",
    "  > Players: 4 players.\n",
    "  > Deck: 48 or 40 cards (the 9's are optional) → each rank appears twice!\n",
    "    - French Deck: Clubs, Spades, Hearts, Diamonds (each suit has A, K, Q, J, 10, 9).\n",
    "    - German Deck: Acorns, Leaves, Hearts, Bells. (each suit has A, K, O, U, 10, 9).\n",
    " > Goal: Win points by taking high-value cards in tricks.\n\n",
    "2. Team's Re/Kontra or Solo\n\n",
    " > Teams are not fixed and are determined during the game:\n",
    "    - Re Team → Players with both Queens of Clubs (♣Q).\n",
    "    - Kontra Team → The other two players.\n",
    "  > Hidden teams: At first, players may not know their partners!\n",
    "  > Auctions: May declare a contract to form a team and increase the score.\n",
    "  > A solo player can declare a Solo round and play alone against the others.\n\n",
    "3. Playing Tricks\n\n",
    " > Each round has 12 to 10 tricks (Ex, for 48 cards / 4 players = 12 rounds).\n",
    " > Players must follow suit if possible.\n",
    " > Highest card wins the trick.\n\n",
    "4. Scoring (Most Important Cards)\n\n",
    " > After a round, each team counts the points of their tricks and bids.\n\n",
    " > To get the winner team, sum the points of the tricks of the team as:\n\n",
    "  ╭────────────────╮\n",
    "  | ", BOLD, "Suit", RESET,"  | ", BOLD, "Points", RESET, " |\n",
    "  |----------------|\n",
    "  | ", BLUE, "Ace", RESET,"   |   11   |\n",
    "  | ", BLUE, "Ten", RESET,"   |   10   |\n",
    "  | ", BLUE, "King", RESET,"  |   4    |\n",
    "  | ", BLUE, "Queen", RESET," |   3    |\n",
    "  | ", BLUE, "Jack", RESET,"  |   2    |\n",
    "  | ", BLUE, "Nine", RESET,"  |   0    |\n",
    "  ╰────────────────╯\n\n",
    "5. Game End & Winning\n\n",
    " > Each team counts points.\n",
    " > A full game is usually to 250 or 500 points.\n",
    " > Bonus points for special moves (Doppelkopf, catching a Fox, etc.).\n",
);

pub const SPECIAL_FEATURES: &str = concatcp!(
    BOLD, UNDERLINE, "\nSpecial features\n\n", RESET,
    " - The following are special features that have an impact on a game:\n",
    "   > ", BLUE, "Catch the fox", RESET, " (Non-solo game): ", GREEN, BOLD, "+1", RESET," a team's ace of diamonds ('the fox') is won by the other team.\n",
    "   > ", BLUE, "Doppelkopf", RESET, " (Non-solo game): ", GREEN, BOLD, "+1", RESET," a team wins a trick with 40 or more points.\n",
    "   > ", BLUE, "Charlie Miller", RESET, " (Non-solo game): ", GREEN, BOLD, "+1", RESET," a team's jack of clubs ('Charlie Miller'/Karlchen Müller') wins the last trick.\n",
    "   > Solo special score: The solo player gets thrice the value added or subtracted.\n\n",
    );

pub const SCORING: &str = concatcp!(
    BOLD, UNDERLINE, "\nScoring\n\n", RESET,
    "After a round, each team counts the points of their tricks and bids.\n\n",
    "To get the winner team, sum the points of the tricks of the team as:\n\n",
    "  ╭────────────────╮\n",
    "  | ", BOLD, "Suit", RESET,"  | ", BOLD, "Points", RESET, " |\n",
    "  |----------------|\n",
    "  | ", BLUE, "Ace", RESET,"   |   11   |\n",
    "  | ", BLUE, "Ten", RESET,"   |   10   |\n",
    "  | ", BLUE, "King", RESET,"  |   4    |\n",
    "  | ", BLUE, "Queen", RESET," |   3    |\n",
    "  | ", BLUE, "Jack", RESET,"  |   2    |\n",
    "  | ", BLUE, "Nine", RESET,"  |   0    |\n",
    "  ╰────────────────╯\n\n",
    BOLD, "Example:\n\n", RESET,
    " - Re played Ace of Clubs + 9 of Clubs: 11(Ace) + 9 (0) = 11 points.\n",
    " - Kontra played 10 of Clubs + 9 of Clubs: 10(10) + 9(0) = 10 points.\n\n",
    "There are no trumps, Ace is the highest suit, so Re team wins the round and all the cards.\n",
    "Re team gains 21 (11 + 10) points.\n\n",
    BOLD, "Adding bids and calculating score:\n\n", RESET,
    "The rounds will continue until all the cards are played. Then game value is calculated as follows:\n",
    " - ", GREEN, BOLD, "1", RESET," for the team that wins the game.\n",
    " - ", GREEN, BOLD, "+1", RESET," if the winning team is Kontra ('gegen die Alten', against the elders) unless a solo is played\n",
    " - ", GREEN, BOLD, "+2", RESET," for an announcement of Re\n",
    " - ", GREEN, BOLD, "+2", RESET," for an announcement of Kontra\n",
    " - ", GREEN, BOLD, "+1", RESET," if the losing team has less than 90 points\n",
    " - ", GREEN, BOLD, "+1", RESET," if No 90 was announced\n",
    " - ", GREEN, BOLD, "+1", RESET," if the winning team won with more than 120 points against an announcement of No 90\n",
    " - ", GREEN, BOLD, "+1", RESET," if the losing team has less than 60 points\n",
    " - ", GREEN, BOLD, "+1", RESET," if No 60 was announced\n",
    " - ", GREEN, BOLD, "+1", RESET," if the winning team won with at least 90 points against an announcement of No 60\n",
    " - ", GREEN, BOLD, "+1", RESET," if the losing team has less than 30 points\n",
    " - ", GREEN, BOLD, "+1", RESET," if No 30 was announced\n",
    " - ", GREEN, BOLD, "+1", RESET," if the winning team won with at least 60 points against an announcement of No 30\n",
    " - ", GREEN, BOLD, "+1", RESET," if the winning team made all tricks\n",
    " - ", GREEN, BOLD, "+1", RESET," if Schwarz was announced\n",
    " - ", GREEN, BOLD, "+1", RESET," if the winning team won with at least 30 points against an announcement of Schwarz\n",
    " - Add special features:\n",
    "   > ", BLUE, "Catch the fox", RESET, " (Non-solo game): ", GREEN, BOLD, "+1", RESET," a team's ace of diamonds ('the fox') is won by the other team.\n",
    "   > ", BLUE, "Doppelkopf", RESET, " (Non-solo game): ", GREEN, BOLD, "+1", RESET," a team wins a trick with 40 or more points.\n",
    "   > ", BLUE, "Charlie Miller", RESET, " (Non-solo game): ", GREEN, BOLD, "+1", RESET," a team's jack of clubs ('Charlie Miller'/Karlchen Müller') wins the last trick.\n",
    "   > Solo special score: The solo player gets thrice the value added or subtracted.\n\n",
    "After applying this rules, the points are added to each member\n\n",
    BOLD, "Example:\n\n", RESET,
    "Re, no 60 was announced, Kontra team said Kontra. Kontra gets 60 points and therefore wins.\n",
    " > Game was won:           +1\n",
    " > Won against the elders: +1\n",
    " > Re was announced:       +2\n",
    " > Kontra was announced:   +2\n",
    " > No 90 was announced:    +1\n",
    " > No 60 was announced:    +1\n",
    "                           =", RED, BOLD, "8 points\n\n", RESET,
    "Both Kontra players get +8, both Re -8.\n"
);
