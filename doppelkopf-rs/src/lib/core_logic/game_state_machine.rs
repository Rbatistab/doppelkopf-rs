//! Helper to handle game state (optional for consuming application)
//!
//! Follows logic as:
//! 1. Take an entry point (either from `new-game` or `join-game`)
//! 2. Follows to waiting time for all players to join
//! 3. Starts a game loop:
//!    3.1 Deal (set player's cards)
//!    3.2 Auction (updates game round's auctions)
//!    3.4 Play trick loop (all players play)
//!    3.5 Apply rules (update scores)
//!    4. Give a winner

use log::debug;
use crate::model::types::game_state::GameState;

#[derive(Debug, PartialEq)]
pub enum GameStateStep {
    Start,
    WaitingForPlayers,
    PlayingRounds,
    GameOver
}

#[derive(Debug, PartialEq)]
pub struct GameStateMachine {
    pub current_game_step: GameStateStep,
    pub game_state: GameState
}

impl GameStateMachine {
    /// Create a new Game State Machine on Initial state
    ///
    /// # Returns
    /// * New GameStateMachine instance on Start step
    ///
    /// # Examples
    /// ```
    /// use dppkf_lib::core_logic::game_state_machine::{GameStateMachine, GameStateStep};
    /// let new_game_state_machine = GameStateMachine::new();
    ///
    /// assert_eq!(new_game_state_machine.current_game_step, GameStateStep::Start);
    // ```
    pub fn new() -> Self {
        debug!("Creating new game state machine...");
        GameStateMachine {
            current_game_step: GameStateStep::Start,
            game_state: GameState::new(),
        }
    }

    /// Steps into next step
    ///
    /// TBD
    pub fn next_state(&mut self) {
        debug!("Transitioning from step {:?}", self.current_game_step);

        self.current_game_step = match self.current_game_step {
            GameStateStep::Start => {
                debug!("Start waiting for players...");
                GameStateStep::WaitingForPlayers
            }
            GameStateStep::WaitingForPlayers => {
                debug!("Start playing rounds...");
                GameStateStep::PlayingRounds
            }
            GameStateStep::PlayingRounds => {
                debug!("Start game over...");
                GameStateStep::GameOver
            }
            GameStateStep::GameOver => {
                debug!("Something");
                GameStateStep::GameOver
            }
        };

    }
}

impl Default for GameStateMachine {
    /// Creates a default instance Game State Machine on Initial state.
    /// It's the same as 'GameStateMachine::new()'
    ///
    /// # Returns
    /// * New GameStateMachine instance on Start step
    ///
    /// # Examples
    /// ```
    /// use dppkf_lib::core_logic::game_state_machine::{GameStateMachine, GameStateStep};
    /// let new_game_state_machine = GameStateMachine::default();
    ///
    /// assert_eq!(new_game_state_machine.current_game_step, GameStateStep::Start);
    // ```
    fn default() -> Self {
        Self::new()
    }
}