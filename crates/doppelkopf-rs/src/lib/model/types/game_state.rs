//! This module represents the state of the game. It tracks the current
//! game status including players, deck, and game identification, but does
//! not implement game logic.


use uuid::Uuid;
use doppelkopf_cards_lib::deck::Deck;
use crate::model::types::player::Player;

/// Represents the current state of a Doppelkopf game
///
/// # Fields
///
/// * `id` - Unique identifier for the game instance (UUID value)
/// * `players` - List of players currently in the game
/// * `deck` - The card deck being used for this game
///
/// # Examples
///
/// ```
/// use dppkf_lib::model::types::game_state::GameState;
/// let game = GameState::new();
/// assert!(game.id.len() > 0);
/// assert!(game.players.is_empty());
/// ```
#[derive(Debug, PartialEq)]
pub struct GameState {
    pub id: String,
    pub players: Vec<Player>,
    pub deck: Deck
}

impl GameState {
    /// creates a new game state with a unique identifier and empty player list
    ///
    /// initializes a new game with:
    /// - a randomly generated uuid v4 as the game identifier
    /// - an empty vector for players
    /// - a default deck of cards
    ///
    /// # returns
    ///
    /// a new gamestate instance with empty players
    ///
    /// # examples
    ///
    /// ```
    /// use doppelkopf_cards_lib::deck::Deck;
    /// use dppkf_lib::model::types::game_state::GameState;
    ///
    /// let game = GameState::new();
    /// assert!(game.id.len() > 0);
    /// assert!(game.players.is_empty());
    /// assert_eq!(game.deck, Deck::default_deck());
    /// ```
    pub fn new() -> GameState {
        Self {
            id: Uuid::new_v4().to_string(),
            players: Vec::new(),
            deck: Deck::default_deck()
        }
    }

    /// Adds a new player to the game
    ///
    /// # Arguments
    ///
    /// * `player` - The Player instance to add to the game
    ///
    /// # Examples
    ///
    /// ```
    /// use dppkf_lib::model::types::game_state::GameState;
    /// use dppkf_lib::model::types::player::{Player, PlayerType};
    ///
    /// let mut game = GameState::new();
    /// let player = Player::from("Alice".to_string(), PlayerType::Human);
    /// game.add_player(player);
    /// assert_eq!(game.players.len(), 1);
    /// ```
    pub fn add_player(&mut self, player: Player) { self.players.push(player); }

    /// Checks if all 4 players joined
    ///
    /// # Return
    /// * True if all there are 4 players in the game
    ///
    /// # Examples
    ///
    /// ```
    /// use dppkf_lib::model::types::game_state::GameState;
    /// use dppkf_lib::model::types::player::{Player,PlayerType};
    ///
    /// let mut game_state = GameState::new();
    /// assert_eq!(game_state.all_players_joined(), false);
    ///
    /// let mock_player_names = ["Martha", "Sabina", "Jasper", "Matthias"];
    /// for name in mock_player_names.iter() {
    ///     game_state.add_player(Player::from(name.parse().unwrap(), PlayerType::AiPlayer));
    /// }
    /// assert_eq!(game_state.all_players_joined(), true);
    /// ```
    pub fn all_players_joined(&self) -> bool {
        self.players.len() == 4
    }
}

/// Creates default game state with a unique identifier and empty player list
/// It's the same as 'GameState::new()'
///
/// Initializes a new game with:
/// - A randomly generated UUID v4 as the game identifier
/// - An empty vector for players
/// - A default deck of cards
///
/// # Returns
///
/// A new GameState instance with empty players
///
/// # Examples
///
/// ```
/// use doppelkopf_cards_lib::deck::Deck;
/// use dppkf_lib::model::types::game_state::GameState;
///
/// let game = GameState::default();
/// assert!(game.id.len() > 0);
/// assert!(game.players.is_empty());
/// assert_eq!(game.deck, Deck::default_deck());
/// ```
impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}