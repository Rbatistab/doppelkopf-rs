//! This module handles states for a single doppelkopf round over all it's phases (deal cards, auction, play tricks, evaluate)
//!
//! TBD

use log::debug;

/// Represents current step of playing one Doppelkopf round
#[derive(Debug, PartialEq)]
pub enum PlayingRoundStep {
    /// Dealing 12 (or 10) cards to each player
    DealingCards,
    /// Playing bids to modify score
    Auctioning,
    /// Play tricks to win the round
    PlayingTricks,
    /// Defining winner (or winner team) of the round
    ApplyingRules
}

/// Handles logic for a state machine on a single round of Doppelkopf
///
/// # Fields
/// * `current_round_step` - Current step of the round
/// # Examples
/// TBD
#[derive(Debug, PartialEq)]
pub struct PlayingRoundStateMachine {
    current_round_step: PlayingRoundStep
    // Should add a round state? or update the game state?
}

impl PlayingRoundStateMachine {
    /// Create a new Round State Machine
    ///
    /// # Returns
    /// * New PlayingRoundStateMachine instance on DealingCards step
    pub fn new() -> Self {
        debug!("Creating new playing round state machine...");
        PlayingRoundStateMachine {
            current_round_step: PlayingRoundStep::DealingCards
        }
    }
}

impl Default for PlayingRoundStateMachine {
    /// Creates default Round State Machine
    /// It's the same as 'PlayingRoundStatemachine::new()'
    ///
    /// # Returns
    /// * New PlayingRoundStateMachine instance on DealingCards step
    fn default() -> Self {
        Self::new()
    }
}

