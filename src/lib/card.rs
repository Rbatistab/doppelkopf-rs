enum StandardCardRank {
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
    ACE,
}

enum Suit {
    SPADES,
    HEARTS,
    DIAMONDS,
    CUBS,
}

struct Card {
    rank: StandardCardRank,
    symbol: Suit,
}
