# doppelkopf-rs

Do you love those moments when playing [Uno](https://www.letsplayuno.com/) turns a card game into a game of thrones episode?
Me too, which is why I bring you this open source version of [doppelkopf game](https://en.wikipedia.org/wiki/Doppelkopf) 

***Disclaimer: grug sees public domain game, grug sees no lawsuits, grub develops game***

Doppelkopf game whole cross-platform application:
* Infrasctrucre: [dopplekopf-cdk](https://github.com/Rbatistab/dopplekopf-cdk/tree/main)
* Frontend application: [dopplekopf-app](https://github.com/Rbatistab/doppelkopf-app)
* Backend: [dopplekopf-rs](https://github.com/Rbatistab/doppelkopf-rs)

## Installation -> Maybe this won't happen

***(When this is in crates.io)***
```
$ cargo install doppelkopf-rs --locked
$ doppelkopf-rs
```

## The game

Doppelkopf is a popular card game in Germany, family of the "trick-tacking" games, based on strategy, teamwork, and tactical decision-making. And since it's German it won't be complicated to understand.
It goes as:
1. [Card deal and contract](#card-deal-and-contract)
1. [Play tricks](#play-tricks)
1. [Make teams](#make-teams)
1. IN PROGRESS


### Card deal and contract

![first_step_card_deal_and_contract](https://drive.google.com/uc?export=view&id=1zV2rW_a4iCs9uYO9FmfmYFaqUFYcwfPq)

On the standard game, you'll have to deal 2 decks (red and black) of 24 (*or 20*) cards each for a total of 48 (*or 40*) cards. Remove all cards with rank lower to 9 to have 48 cards, consisting of:
* 4 Suites (French or [German](https://en.wikipedia.org/wiki/German-suited_playing_cards) deck):
  * ♦ Diamonds (*or Bells 🇩🇪*)
  * ❤ Hearts (*german and french*)
  * ♠ Spades (*or Leaves 🇩🇪*)
  * ♣ Clubs (*or Acorns 🇩🇪*)
* Each suit has 6 possible ranks (2 cards of each suit per rank or *'Doppelkopf'*):
***If you want to play with 40 cards just remove the nines.***

Start by dealing 12 cards to each player (*10 for 40-card games*).The dealer is the first player to deal, and dealing passes clockwise.

After this, you'll need to agree a contract, also known as auction. In this phase, the players either agree to a [normal-game](#normal_game) by saying "*Fine*" (*Gesund 🇩🇪*) or one or more of the players call "*Hold*" (*Halt 🇩🇪*) to call a [wedding](#wedding) or a [solo](#solo)

***Always two there are, no more, no less.*** Once the contract is set, there will be two teams of 2 vs 2 or 1 vs 3.

#### Normal game

When 2 different players have a Queens of Clubs (or *Obers* 🇩🇪, known as *Die Alten*, the elders") they are a team (**Re**) and will play against the other 2 (**Kontra**):
* `Re` team achieves at least 121 points (*or a different pre-agreed condition*) -> `Re` wins
* `Re` fails to get the points -> `Kontra` wins
On a normal game, this is the trump, that starts with the 10 of hearts, called *Dulle* (or *Tolle* 🇩🇪):

❤️ 10 | ♣️ Q | ♠️ Q | ❤️ Q | ♦️ Q | ♣️ J | ♠️ J | ❤️ J | ♦️ J | ♦️ A | ♦️ 10 | ♦️ K | ♦️ 9

#### Wedding

A player (*suitor*) that has both Queens of Clubs can call a "wedding" and form a **Re** team with the first player to win a trick. However, if the *suitor* wons the first three games, he/she should play a *Diamond [solo](#solo)* against the other three. Also, a *suitor* can chose not to say "wedding" on the contract and play a *Diamond [solo](#solo)* too, called *Silent Solo* (*Stilles Solo 🇩🇪*).

Wedding uses the same Trump as the [normal game](#normal-game)

#### Solo

***May you ride eternal, shiny and chrome.*** A single player will play against the other three. He/She will earn trice the value of the score card in case of winning a trick or lose trice the value on the other case, there are:
* ♦ Diamond Solo: Same Trump as the [normal game](#normal-game)
* Jack Solo (*Bubensolo 🇩🇪*): Only Jacks make a Trump
* Queen Solo (*Damensolo 🇩🇪*): Only Queen make a Trump
* Ace Solo (*Fleischloser/Knochenmann 🇩🇪*): There are no Trumps
* Suit Solo (*Farbensolo 🇩🇪*): Announce a Suit to be a Trump for Jacks and Queens


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


    | Card | Points per card |
    |------|-----------------|
    | Aces (*Deuces*) | 11 |
    | Tens | 10 |
    | Kings | 4 |
    | Queens (*Obers*) | 3 |
    | Jacks (*Unters*) | 2 |
    | Nines | 0 |

### Special features

* **Doppelkopf**: A trick is worth double points if it contains 40 or more points (usually means two Aces are in the trick).

## Game sources:

* [Wikipedia](https://en.wikipedia.org/wiki/Doppelkopf)
* https://www.pagat.com/schafkopf/doko.html
* https://boardgamegeek.com/blogpost/127676/doppelkopf-20-a-brilliant-traditional-trick-taker
* https://www.deck-of-cards.com/doppelkopf.html
