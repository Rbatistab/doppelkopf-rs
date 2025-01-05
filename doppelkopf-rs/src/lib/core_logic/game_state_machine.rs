//! Is the CLI handler of the game
//! Follows logic as:
//! 1. Take an entry point (either from `new-game` or `join-game`)
//! 2. Follows to waiting time for all players to join
//! 3. Starts a game loop:
//!  3.1 Deal (set player's cards)
//!  3.2 Auction (updates game round's auctions)
//!  3.4 Play trick loop (all players play)
//!  3.5 Apply rules (update scores)
//! 4. Give a winner

#[derive(Debug, PartialEq)]
enum GameState {
    Start,
    WaitingForPlayers,
    Dealing,
    Auctioning,
    PlayingTricks,
    GameOver
}

pub struct StateMachine {
    current_state: GameState,
}

impl StateMachine {
   /// Create a new State Machine on Initial state
   ///
   /// # Returns
   /// * New StateMachine instance
   pub fn new() -> Self {
      StateMachine {
          current_state: GameState::Start
      }
   }
}