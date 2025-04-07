# doppelkopf-rs

This is a CLI implementation of the German Card game [Doppekkopf](https://en.wikipedia.org/wiki/Doppelkopf) and the backend on a larger end-to-end project:
* Infrastructure: [dopplekopf-cdk](https://github.com/Rbatistab/dopplekopf-cdk/tree/main)
* Frontend application: [dopplekopf-app](https://github.com/Rbatistab/doppelkopf-app)
* Backend: [dopplekopf-rs](https://github.com/Rbatistab/doppelkopf-rs)

## Installation 

Clone the package and follow the installer:
```shell
# Clone and build the package
git clone https://github.com/Rbatistab/doppelkopf-rs.git
cd doppelkopf-rs
# Install
make install
# Clean up
cd ../
rm -rf doppelkopf-rs
```

To uninstall clone the package and follow the "uninstaller":
```shell
# Clone the package
git clone https://github.com/Rbatistab/doppelkopf-rs.git
cd doppelkopf-rs
# Uninstall
make uninstall
# Clean up
cd ../
rm -rf doppelkopf-rs
```

## Usage

Start by running `dppkf -h` to check the available options.

### 1. Start or join a new game [IN_PROGRESS]

To start a new game just run the `new-game` command and follow the options
```shell
# Start a new game with interactive mode
$ dppkf new-game 
````

To join an existing game, just follow with the `join-game` command:
```shell
# Start a new game with interactive mode
$ dppkf join-game 
````

### 2. [IN PROGRESS]
TBD

### 3. Not sure on something about Doppelkopf? Use the cheat sheet! [IN_PROGRESS]

Run `dppkf cheat-sheet` to get an interactive prompt and navigate to the cheat sheet that you need:
```shell
$ dppkf cheat-sheet
? Select a cheat:
> Game overview
  Card deal
  Contracts
  Tricks
  Bids
  Trumps
v Rules
[Use ↑↓ arrows to navigate, enter to select]
```

You can also skip the interactive prompt and jump to the cheat with the `-c` flag:
```shell
$ dppkf cheat-sheet -c 

Bids

A player may make announcements claiming that their team will succeed in achieving a specific goal.
These announcements increase the game value regardless of whether they are fulfilled. However, when a team fails a self-given goal, they automatically lose.

The bids, in order, are:
 - Re/Contra: The name of the player's team, claims that the team will make more than 120 points. and undertaking that their team will score more than 120 points. All following bids should follow this bid, this is the first valid bid => Doubles the value
    > Can only be played with 11 cards left
 - No 90 (also called 'No 9'): Claims the opponents will get less than 90 points => Triples the value
    > Can only be played with 10 cards left
 - No 60 (also called 'No 6'): Claims the opposing team will not make 60 points => X4 the value
    > Can only be played with 9 cards left
 - No 30 (also called 'No 3': You get the idea
    > Can only be played with 8 cards left
 - Schwarz: Claims the opponents will not get a single trick, not even a trick worth zero points
    > Can only be played with 7 cards left

The fun part here is that each bid will imply all previous: 'No 6' will imply 'No 9' which will imply 'Re/Contra', hence the game increases by 4.
Also, every bid can have a counterbid making both valid.

```

Options:
* game-overview 
* card-deal
* contracts
* tricks
* bids
* trumps
* rules
* special-features
* scoring

Example:
```shell
# I forgot the trumps!
$ dppfk cheat-sheet -c trumps
```

# How to play the game?

Doppelkopf is a popular card game in Germany, family of the "trick-tacking" games, based on strategy, teamwork, and tactical decision-making. And since it's German it won't be complicated to understand.

## 1. Card deal and contract

![first_step_card_deal_and_contract](https://drive.google.com/uc?export=view&id=15rBJbUubBfbccd2OJ7pha5I9s-Dr5Ql6)

On the standard game, you'll have to deal 2 decks (red and black) of 24 (*or 20*) cards each for a total of 48 (*or 40*) cards. Remove all cards with rank lower to 9 to have 48 cards, consisting of:
* 4 Suites (French or [German](https://en.wikipedia.org/wiki/German-suited_playing_cards) deck):
    * ♦ Diamonds (*or Bells 🇩🇪*)
    * ❤ Hearts (*both 🇩🇪 and 🇫🇷*)
    * ♠ Spades (*or Leaves 🇩🇪*)
    * ♣ Clubs (*or Acorns 🇩🇪*)
* Each suit has 6 possible ranks (2 cards of each suit per rank or *'Doppelkopf'*):
  ***If you want to play with 40 cards just remove the nines.***

Start by dealing 12 cards to each player (*10 for 40-card games*).The dealer is the first player to deal, and dealing passes clockwise.

## 2. Contract

After this, you'll need to agree a contract, also known as auction. In this phase, the players either agree to a [normal game](#normal_game) by saying "*Fine*" (*Gesund 🇩🇪*) or one or more of the players call "*Hold*" (*Halt 🇩🇪*) to call a [wedding](#wedding) or a [solo](#solo).

***Always two there are, no more, no less.*** Once the contract is set, there will be two teams of 2 vs 2 or 1 vs 3.

### 2.1 Normal game

When 2 different players have a Queens of Clubs (or *Obers* 🇩🇪, known as *Die Alten*, the "elders") they are a team (**Re**) and will play against the other 2 (**Kontra**):
* `Re` team achieves at least 121 points (*or a different pre-agreed condition*) -> `Re` wins
* `Re` fails to get the points -> `Kontra` wins

### 2.2 Wedding

A player (*suitor*) that has both Queens of Clubs can call a "wedding" and form a **Re** team with the first player to win a trick. However, if the *suitor* wons the first three games, he/she should play a *Diamond [solo](#solo)* against the other three. Also, a *suitor* can chose not to say "wedding" on the contract and play a *Diamond [solo](#solo)* too, called *Silent Solo* (*Stilles Solo 🇩🇪*).

### 2.3 Solo

***May you ride eternal, shiny and chrome.*** A single player will play against the other three. He/She will earn trice the value of the score card in case of winning a trick or lose trice the value on the other case, there are:
* Diamond Solo: Only Diamonds make a Trump
* Jack Solo (*Bubensolo 🇩🇪*): Only Jacks make a Trump
* Queen Solo (*Damensolo 🇩🇪*): Only Queen make a Trump
* Ace Solo (*Fleischloser/Knochenmann 🇩🇪*): There are no Trumps
* Suit Solo (*Farbensolo 🇩🇪*): Announce a Suit to be a Trump for Jacks and Queens

## 3. Play tricks and bids

[IMAGE IN PROGRESS]

Doppelkopf is won by playing tricks to score points in a team. **The default is to win a round by scoring 121 points or more**, however it's possible to pre-agree a different score or a number of rounds.

Points are scored based on the number and value of the tricks won. Some specific card combinations or achievements can also earn extra points.

### 3.1 Play a trick

The game flows like this:

1. The player to the left of the dealer plays the first trick (normally the highest at hand)
1. Players must follow suit if possible. There are 3 options:
    * Play a trump
    * Play any card
    * Discard
1. After all tricks are played the round is won by:
    1. The highest trump
    1. The highest card of the suit led. Since each card exists twice, there is the possibility of a tie; in that case, the first-played card wins the trick

### 3.2 Bids

A player can complement the trick with a bid or announcement. The bid claims that their team will accomplish certain goal. Bids increase the game value regardless of whether they are fulfilled. By failing a self-given goal, the team automatically lose.

The bids, in order, are:
1. "Re"/"Contra": The name of the player's team, claims that the team will make more than 120 points.
   and undertaking that their team will score more than 120 points. ***All following bids should follow this bid, this is the first valid bid*** => Doubles the value
    * Can only be played with 11 cards left
1. "No 90" (also called "No 9"): Claims the opponents will get less than 90 points => Triples the value
    * Can only be played with 10 cards left
1.  "No 60" (also called "No 6"): Claims the opposing team will not make 60 points => X4 the value
    * Can only be played with 9 cards left
1. "No 30" (also called "No 3": You get the idea
    * Can only be played with 8 cards left
1. "Schwarz": Claims the opponents will not get a single trick, not even a trick worth zero points
    * Can only be played with 7 cards left

The fun part here is that each bid will imply all previous: "No 6" will imply "No 9" which will imply "Re/Contra", hence the game increases by 4.
Also, every bid can have a counterbid making both valid.

## 4. Evaluate Rules (Scoring)

The game rules will determine the winner of the round and the amount of points that each team wins or loses.

### 4.1 Trumps

**Normal game:** Starts with the 10 of hearts, called *Dulle* (or *Tolle* 🇩🇪):

❤️ 10 | ♣️ Q | ♠️ Q | ❤️ Q | ♦️ Q | ♣️ J | ♠️ J | ❤️ J | ♦️ J | ♦️ A | ♦️ 10 | ♦️ K | ♦️ 9

**Wedding:** Same Trump as the [normal game](#normal-game)

**Solos:**
* Diamond Solo: Same Trump as the [normal game](#normal-game)
* Jack Solo (*Bubensolo 🇩🇪*): Only Jacks make a Trump
* Queen Solo (*Damensolo 🇩🇪*): Only Queen make a Trump
* Ace Solo (*Fleischloser/Knochenmann 🇩🇪*): There are no Trumps
* Suit Solo (*Farbensolo 🇩🇪*): Announce a Suit to be a Trump for Jacks and Queens

### 4.2 Special features (extra points)

***There are no extra points for Solo games***

* **Doppelkopf**: Get an extra point if a trick has 40 or more points
* **Charlie Miller**: Get an extra point if a team's jack of clubs wins the last trick

### 4.3 Scoring

Game score and round winner:
1. The player that won gets all the cards, which won't be played again. The card points taken in the tricks are counted and each player in the winning team gets the game points added to their score, while the losing players have that value deducted.
1. The winner of the trick leads the next one

   | Card | Points per card |
   |------|-----------------|
   | Aces (*Deuces*) | 11 |
   | Tens | 10 |
   | Kings | 4 |
   | Queens (*Obers*) | 3 |
   | Jacks (*Unters*) | 2 |
   | Nines | 0 |



Score of each player:

* Add game value to score of each player of the wininng team
* Substract game value to score of each player of the losing team
* If a Solo wins she gets thrice added/substracted (ouch!)

References:
* [Wikipedia](https://en.wikipedia.org/wiki/Doppelkopf)
* https://www.pagat.com/schafkopf/doko.html
* https://boardgamegeek.com/blogpost/127676/doppelkopf-20-a-brilliant-traditional-trick-taker
* https://www.deck-of-cards.com/doppelkopf.html
