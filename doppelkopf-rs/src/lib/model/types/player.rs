//! Representation of a player on the game
//!
//! Core player types and implementations to manage players in a Doppelkopf game

use uuid::Uuid;

/// Represents if the player is human or AI
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
    /// use dppkf_lib::model::types::player::Player;
    /// let new_player = Player::new();
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
    /// let player_name = String::from("T100");
    /// let new_player = Player::from(player_name, PlayerType::AiPlayer);
    /// let human_player_name = String::from("Sarah Connor");
    /// let human_new_player = Player::from(human_player_name, PlayerType::Human);
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

#[cfg(test)]
mod player_tests {
    use super::*;

    #[test]
    fn test_new_player_from() {
        let human_player_1_name = String::from("Sarah Connor");

        let human_player_1 = Player::from(human_player_1_name.clone(), PlayerType::Human);

        assert!(!human_player_1.id.is_empty());
        assert_eq!(human_player_1.name, human_player_1_name);
        assert_eq!(human_player_1.score, 0);
        assert_eq!(human_player_1.player_type, PlayerType::Human);


        let ai_player_name = String::from("T1000");

        let ai_player = Player::from(ai_player_name.clone(), PlayerType::AiPlayer);

        assert!(!ai_player.id.is_empty());
        assert_eq!(ai_player.name, ai_player_name);
        assert_eq!(ai_player.score, 0);
        assert_eq!(ai_player.player_type, PlayerType::AiPlayer);
    }
}