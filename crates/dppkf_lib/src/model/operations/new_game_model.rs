/// Model operations for `new_game_logic`

use dppkf_cards_lib::deck::PackSize;
use dppkf_cards_lib::suits::SuitType;
use crate::model::types::player::Player;

/// Model input for `new_game_logic` from the dppkf_lib
///
/// # Fields
/// * `player` - Instance of Player. Represents the player creating the game
/// * `suit_type` - Instance of SuitType for the new game
/// * `pack_size` - Enum value of PackSize for new game, represents 40 or 48
#[derive(Debug)]
pub struct NewGameInput {
    pub player: Player,
    pub suit_type: SuitType,
    pub pack_size: PackSize
}

impl NewGameInput {
    /// Contains the arguments to create a new game. Meant to be consumed by new_game_logic
    ///
    /// # Fields
    /// * `player` - Instance of Player. Represents the player creating the game
    /// * `suit_type` - Instance of SuitType for the new game
    /// * `pack_size` - Enum value of PackSize for new game, represents 40 or 48
    ///
    /// # Returns NewGameInput instance
    ///
    /// # Example
    /// ```
    /// use dppkf_lib::model::operations::new_game_model::NewGameInput;
    /// use dppkf_lib::model::types::player::{Player, PlayerType};
    /// use dppkf_cards_lib::suits::SuitType;
    /// use dppkf_cards_lib::deck::PackSize;
    ///
    /// let player = Player::from("Hans".to_string(), PlayerType::AiPlayer);
    /// let suit_type = SuitType::German;
    /// let pack_size = PackSize::Forty;
    ///
    /// let new_game_args = NewGameInput::from(player, suit_type, pack_size);
    /// assert_eq!(new_game_args.player.name, "Hans".to_string());
    /// assert_eq!(new_game_args.suit_type, SuitType::German);
    /// assert_eq!(new_game_args.pack_size, PackSize::Forty);
    /// ```
    pub fn from(player: Player, suit_type: SuitType, pack_size: PackSize) -> NewGameInput {
        NewGameInput {
            player,
            suit_type,
            pack_size
        }
    }

    /// Contains the arguments to create a new game. Meant to be consumed by new_game_logic
    ///
    /// # Returns NewGameInput instance
    ///
    /// # Example
    /// ```
    /// use dppkf_lib::model::operations::new_game_model::NewGameInput;
    /// use dppkf_lib::model::types::player::Player;
    /// use dppkf_cards_lib::suits::SuitType;
    /// use dppkf_cards_lib::deck::PackSize;
    ///
    /// let new_game_args = NewGameInput::new();
    /// assert_eq!(new_game_args.player, Player::new());
    /// assert_eq!(new_game_args.suit_type, SuitType::French);
    /// assert_eq!(new_game_args.pack_size, PackSize::FortyEight);
    /// ```
    pub fn new() -> NewGameInput {
        NewGameInput {
            player: Player::new(),
            suit_type: SuitType::French,
            pack_size: PackSize::FortyEight
        }
    }

    /// Sets a new player instance on 'player' field
    ///
    /// # Example
    /// ```
    /// use dppkf_lib::model::operations::new_game_model::NewGameInput;
    /// use dppkf_lib::model::types::player::{Player, PlayerType};
    ///
    /// let new_player = Player::from("Grettel".to_string(), PlayerType::AiPlayer);
    /// let mut new_game_args = NewGameInput::new();
    /// new_game_args.set_player(new_player);
    ///
    /// assert_eq!(new_game_args.player.name, "Grettel".to_string());
    /// ```
    pub fn set_player(&mut self, player: Player) {
        self.player = player;
    }

    /// Sets a SuitType enum value on 'suit_type' field
    ///
    /// # Example
    /// ```
    /// use dppkf_lib::model::operations::new_game_model::NewGameInput;
    /// use dppkf_cards_lib::suits::SuitType;
    ///
    /// let mut new_game_args = NewGameInput::new();
    /// new_game_args.set_suit_type(SuitType::French);
    ///
    /// assert_eq!(new_game_args.suit_type, SuitType::French);
    /// ```
    pub fn set_suit_type(&mut self, suit_type: SuitType)  {
        self.suit_type = suit_type;
    }

    /// Sets a PackSize enum value on 'pack_size' field
    ///
    /// # Example
    /// ```
    /// use dppkf_lib::model::operations::new_game_model::NewGameInput;
    /// use dppkf_cards_lib::deck::PackSize;
    ///
    /// let mut new_game_args = NewGameInput::new();
    /// new_game_args.set_pack_size(PackSize::FortyEight);
    ///
    /// assert_eq!(new_game_args.pack_size, PackSize::FortyEight);
    /// ```
    pub fn set_pack_size(&mut self, pack_size: PackSize)  {
        self.pack_size = pack_size;
    }
}

impl Default for NewGameInput {
    /// Creates default object to handle the arguments to create a new game. Meant to be consumed
    /// by new_game_logic.
    /// It's the same as 'NewGameInput::new()'
    ///
    /// # Returns NewGameInput instance
    ///
    /// # Example
    /// ```
    /// use dppkf_lib::model::operations::new_game_model::NewGameInput;
    /// use dppkf_lib::model::types::player::Player;
    /// use dppkf_cards_lib::suits::SuitType;
    /// use dppkf_cards_lib::deck::PackSize;
    ///
    /// let new_game_args = NewGameInput::default();
    /// assert_eq!(new_game_args.player, Player::new());
    /// assert_eq!(new_game_args.suit_type, SuitType::French);
    /// assert_eq!(new_game_args.pack_size, PackSize::FortyEight);
    /// ```
    fn default() -> Self {
        Self::new()
    }
}