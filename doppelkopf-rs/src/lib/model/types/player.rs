//! Representation of a player on the game
//!
//! Core player types and implementations to manage players in a Doppelkopf game

use uuid::Uuid;

/// Represents if the player is human or AI
#[derive(Debug, PartialEq, Clone)]
pub enum PlayerType {
    /// Human-controlled player that requires user input for moves
    Human,
    /// AI-controlled  player
    AiPlayer
}

/// Represents a player in the game
///
/// # Fields
///
/// * `id` -  An Id set by the game
/// * `name` - The player's display name
/// * `score` - Current score of the player
/// * `player-type` - Tells if the player is human or AI
///
/// # Examples
/// ```
/// use dppkf_lib::model::types::player::{Player, PlayerType};
/// let player = Player {
///     id: String::from("my-id"),
///     name: String::from("Sarah"),
///     score: 0,
///     player_type: PlayerType::Human
/// };
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub score: u32,
    pub player_type: PlayerType
}

impl Player {
    /// Creates a new player, default to human
    ///
    /// # Returns Player instance
    ///
    /// # Examples
    /// ```
    /// use dppkf_lib::model::types::player::{Player, PlayerType};
    ///
    /// let new_player = Player::new();
    /// assert_eq!(new_player.player_type, PlayerType::Human);
    /// ```
    pub fn new() -> Player {
        Player {
            id: String::new(),
            name: String::new(),
            score: 0,
            player_type: PlayerType::Human
        }
    }

    /// Creates a new player, from a name and type
    ///
    /// # Returns Player instance
    ///
    /// # Examples
    /// ```
    /// use dppkf_lib::model::types::player::{Player, PlayerType};
    ///
    /// let ai_player_name = String::from("T100");
    /// let ai_new_player = Player::from(ai_player_name.clone(), PlayerType::AiPlayer);
    /// assert!(!ai_new_player.id.is_empty());
    /// assert_eq!(ai_new_player.name, ai_player_name);
    /// assert_eq!(ai_new_player.player_type, PlayerType::AiPlayer);
    ///
    /// let human_player_name = String::from("Sarah Connor");
    /// let human_new_player = Player::from(human_player_name.clone(), PlayerType::Human);
    /// assert_eq!(human_new_player.player_type, PlayerType::Human);
    /// assert_eq!(human_new_player.score, 0);
    /// ```
    pub fn from(name: String, player_type: PlayerType) -> Player {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            score: 0,
            player_type
        }
    }
}

impl Default for Player {
    /// Creates default new player (human)
    /// It's the same as 'Player::new()'
    ///
    /// # Returns Player instance
    ///
    /// # Examples
    /// ```
    /// use dppkf_lib::model::types::player::{Player, PlayerType};
    ///
    /// let new_player = Player::default();
    /// assert_eq!(new_player.player_type, PlayerType::Human);
    /// ```
    fn default() -> Self {
        Self::new()
    }

}
