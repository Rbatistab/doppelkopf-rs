# doppelkopf-rs

Do you love those moments when playing [Uno](https://www.letsplayuno.com/) turns a card game into a game of thrones episode?
Me too, which is why I bring you this open source version of [doppelkopf game](https://en.wikipedia.org/wiki/Doppelkopf) 

***Disclaimer: grug sees public domain game, grug sees no lawsuits, grub develops game***

## Installation

***(When this is in crates.io)***
```
$ cargo install doppelkopf-rs --locked
$ doppelkopf-rs
```

## The game

Doppelkopf is a popular card game in Germany, family of the "trick-tacking" games, based on strategy, teamwork, and tactical decision-making. And since it's German it won't be complicated to understand.
It goes as:
1. [Card deal](#card-deal)
1. [Play tricks](#play-tricks)
1. [Make teams](#make-teams)
1. IN PROGRESS


### Card deal

On the standard game, you'll have to deal 2 decks (red and black) of 24 (*or 20*) cards each for a total of 48 (*or 40*) cards. Remove all cards with rank lower to 9 to have 48 cards:
* 4 Suites (French or [German](https://en.wikipedia.org/wiki/German-suited_playing_cards) deck):
  * ♦ Diamonds (*or Bells for german*)
  * ❤ Hearts (*german and french*)
  * ♠ Spades (*or Leaves for german*)
  * ♣ Clubs (*or Acorns for german*)
* Each suit has 6 possible ranks (2 cards of each suit per rank or *'Doppelkopf'*):
    | Card | Points per card |
    |------|-----------------|
    | Aces (*Deuces*) | 11 |
    | Tens | 10 |
    | Kings | 4 |
    | Queens (*Obers*) | 3 |
    | Jacks (*Unters*) | 2 |
    | Nines | 0 |

***If you want to play with 40 cards, remove the nines, no body wants them anyway.***

Start by dealing 12 cards to each player (*10 for 40-card games*).The dealer is the first player to deal, and dealing passes clockwise.

### Play tricks

Doppelkopf is won by playing tricks to score points in a team. **The default is to win a round by scoring 121 points or more**, however it's possible to pre-agree a different score or a number of rounds.

Points are scored based on the number and value of the tricks won. Some specific card combinations or achievements can also earn extra points.

The game flows like this:

1. The player to the left of the dealer leads the first trick. This is, play a card which normally will be the highest at hand.
1. Players must follow suit if possible there are 3 options:
    * Play a [trump](#trumps)
    * Play any card
    * Discard 
1. The trick is won, in order, by:
    1. The highest trump 
    1. The highest card of the suit led. Since each card exists twice, there is the possibility of a tie; in that case, the first-played card wins the trick
1. The player that won gets all the cards, which won't be played again. The card points taken in the tricks are counted and each player in the [winning team](#form-teams) gets the game points added to their score, while the losing players have that value deducted.
1. The winner of the trick leads the next one 

#### Trumps

On a normal game, this is the trump, that starts with the 10 of hearts (called *Dulle* or *Tolle*)

❤️ 10 | ♣️ Q | ♠️ Q | ❤️ Q | ♦️ Q | ♣️ J | ♠️ J | ❤️ J | ♦️ J | ♦️ A | ♦️ 10 | ♦️ K | ♦️ 9

### Form teams

You'll form 2 teams which normally are not known from the start. Teams can change from round to round.

🌰In a nutshell you'll have 2 teams, `Contra` and team `Re`. ***Always two there are, no more, no less.***
* `Re` team achieves at least 121 points (*or a different pre-agreed condition*) -> `Re` wins
* `Re` fails to get the points -> `Contra` wins

In the normal game, the players who hold the queens of clubs (Die Alten = "the old women" or "the elders") or Obers of acorns constitute Re, while the other two are Kontra. In these games, the actual teams are not known from the start. When a player has both queens of clubs or Obers of acorns, that player declares a Wedding (Hochzeit). 


### Special features

* **Re and Kontra**: At the beginning of each round, players can declare "Re" or "Kontra" to double the points at stake. 
* **Doppelkopf**: A trick is worth double points if it contains 40 or more points (usually means two Aces are in the trick).
* **Piglets and Marriage**: Some versions of the game include additional rules like "Piglets" (special scoring bonuses) and "Marriage" (a player holding both Queens of Clubs can declare a marriage to determine the teams for that round).

## Game sources:

* [Wikipedia](https://en.wikipedia.org/wiki/Doppelkopf)
* https://www.pagat.com/schafkopf/doko.html
* https://boardgamegeek.com/blogpost/127676/doppelkopf-20-a-brilliant-traditional-trick-taker
