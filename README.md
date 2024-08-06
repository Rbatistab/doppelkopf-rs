# doppelkopf-rs

Do you love those moments when playing [Uno](https://www.letsplayuno.com/) turns a card game into a game of thornes episode?
Me too, which is why I bring you this open source version of [doppelkopf game](https://en.wikipedia.org/wiki/Doppelkopf) 

***Disclaimer: grug sees public domain game, grug sees no lawsuits, grub develops***

## Installation

***(When this is in crates.io)***
```
$ cargo install doppelkopf-rs --locked
$ doppelkopf-rs
```

## The game

Doppelkopf is a popular card game in Germany, family of the "trick-tacking" games based on strategy, teamwork, and tactical decision-making. And since it's German it won't be complicated to understand.

### Rules

* Win by doing tricks and scoring points: You'll previously agree the points(or rounds too). *Defaults to 121 points*.
* Dynamic teams: You'll form 2 teams which may or not be known at the start, and can change members from round to round.
* This game is played with a double deck of cards:
  * 8 players: 2 x 32 (*Type I*) 
  * 6 players: 2 x 24 (*Type H*)
  * 4 players: 2 x 24 cards (*Type G*) -> Each player is dealt 12 cards
* A dealer is the first to deal, then the dealing passes clockwise

### Gameplay -> [IN PROGRESS]

1. The player to the left of the dealer leads the first trick.
1. Players must follow suit if possible; if not, they can play any card.
1. The trick is won by the highest trump or the highest card of the suit led.
1. The winner of the trick leads the next one.

🌰In a nutshell you'll have 2 teams, `Contra` and team `Re`. ***Always two there are, no more, no less.***
* `Re` team achieves 121 or more points -> `Re` wins
* `Re` fails to get the points -> `Contra` wins

Scoring: Points are scored based on the number and value of the tricks won. Some specific card combinations or achievements can also earn extra points.

### Special features

* **Re and Kontra**: At the beginning of each round, players can declare "Re" or "Kontra" to double the points at stake. 
* **Doppelkopf**: A trick is worth double points if it contains 40 or more points (usually means two Aces are in the trick).
* **Piglets and Marriage**: Some versions of the game include additional rules like "Piglets" (special scoring bonuses) and "Marriage" (a player holding both Queens of Clubs can declare a marriage to determine the teams for that round).

### Pack dealing

You have a card pack of 48 cards, from which you remove all cards bellow rank 9 and have:
* 4 Suites (French or German):
  * ♦ Diamonds (or Bells for german)
  * ❤ Hearts (german and french)
  * ♠ Spades (or Leaves for german)
  * ♣ Clubs (or Acorns for german)
* Each suit has 6 possible ranks (2 cards of each suit per rank or 'Doppelkopf'):

| Card | Points per card |
|------|-----------------|
| Aces(Deuces) | 11 |
| Tens | 10 |
| Kings | 4 |
| Queens(Obers) | 3 |
| Jacks(Unters) | 2 |
| Nines | 0 |

***If you play with 40 cards, remove the nines, no body wants them anyway.***

Start by dealing 12 cards to each player (10 if 40 cards).
Cards: The game uses 48 cards, comprising two decks of 24 cards (9, 10, Jack, Queen, King, Ace in each of the four suits: Diamonds, Hearts, Spades, Clubs).
