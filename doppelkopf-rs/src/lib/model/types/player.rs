use uuid::Uuid;

/// Represents if the player is human or AI
#[derive(Debug, PartialEq)]
pub enum PlayerType {
    /// Player type hasn't being defined
    Undefined,
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
#[derive(Debug, PartialEq)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub score: u32,
    pub player_type: PlayerType
}

impl Player {
    pub fn new() -> Player {
        Player {
            id: String::new(),
            name: String::new(),
            score: 0,
            player_type: PlayerType::Undefined
        }
    }
    pub fn human_player_from_name(name: String) -> Player {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            score: 0,
            player_type: PlayerType::Human
        }
    }

    pub fn ai_player_from_name(name: String) -> Player {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            score: 0,
            player_type: PlayerType::AiPlayer
        }
    }
}

#[cfg(test)]
mod player_tests {
    use super::*;

    #[test]
    fn test_get_human_player() {
        let player_name = String::from("Sarah Connor");

        let player = Player::human_player_from_name(player_name.clone());

        assert!(!player.id.is_empty());
        assert_eq!(player.name, player_name);
        assert_eq!(player.score, 0);
        assert_eq!(player.player_type, PlayerType::Human);
    }

    #[test]
    fn test_ai_player() {
        let player_name = String::from("T1000");

        let player = Player::ai_player_from_name(player_name.clone());

        assert!(!player.id.is_empty());
        assert_eq!(player.name, player_name);
        assert_eq!(player.score, 0);
        assert_eq!(player.player_type, PlayerType::AiPlayer);
    }
}