use doppelkopf_cards_lib::deck::PackSize;
use doppelkopf_cards_lib::suits::SuitType;
use crate::model::types::player::Player;

/// Model for new game
///
/// # Fields
/// * `player` - Instance of Player. Represents the player creating the game
/// * `suit_type` - Instance of SuitType for the new game
/// * `pack_size` - Enum value of PackSize for new game, represents 40 or 48
#[derive(Debug)]
pub struct NewGameArgs {
    pub player: Player,
    pub suit_type: SuitType,
    pub pack_size: PackSize
}

impl NewGameArgs {
    /// Contains the arguments to create a new game. Meant to be consumed by new_game_logic
    ///
    /// # Fields
    /// * `player` - Instance of Player. Represents the player creating the game
    /// * `suit_type` - Instance of SuitType for the new game
    /// * `pack_size` - Enum value of PackSize for new game, represents 40 or 48
    ///
    /// # Returns NewGameArgs instance
    ///
    /// # Example
    /// ```
    /// use dppkf_lib::model::operations::new_game_model::NewGameArgs;
    /// use dppkf_lib::model::types::player::{Player, PlayerType};
    /// use doppelkopf_cards_lib::suits::SuitType;
    /// use doppelkopf_cards_lib::deck::PackSize;
    ///
    /// let player = Player::from("Hans".to_string(), PlayerType::AiPlayer);
    /// let suit_type = SuitType::German;
    /// let pack_size = PackSize::Forty;
    ///
    /// let new_game_args = NewGameArgs::from(player, suit_type, pack_size);
    /// assert_eq!(new_game_args.player.name, "Hans".to_string());
    /// assert_eq!(new_game_args.suit_type, SuitType::German);
    /// assert_eq!(new_game_args.pack_size, PackSize::Forty);
    /// ```
    pub fn from(player: Player, suit_type: SuitType, pack_size: PackSize) -> NewGameArgs {
        NewGameArgs {
            player,
            suit_type,
            pack_size
        }
    }

    /// Contains the arguments to create a new game. Meant to be consumed by new_game_logic
    ///
    /// # Returns NewGameArgs instance
    ///
    /// # Example
    /// ```
    /// use dppkf_lib::model::operations::new_game_model::NewGameArgs;
    /// use dppkf_lib::model::types::player::Player;
    /// use doppelkopf_cards_lib::suits::SuitType;
    /// use doppelkopf_cards_lib::deck::PackSize;
    ///
    /// let new_game_args = NewGameArgs::new();
    /// assert_eq!(new_game_args.player, Player::new());
    /// assert_eq!(new_game_args.suit_type, SuitType::French);
    /// assert_eq!(new_game_args.pack_size, PackSize::FortyEight);
    /// ```
    pub fn new() -> NewGameArgs {
        NewGameArgs {
            player: Player::new(),
            /// Defaults to French suit
            suit_type: SuitType::French,
            /// Defaults to 48 cards
            pack_size: PackSize::FortyEight
        }
    }

    /// Sets a new player instance on 'player' field
    ///
    /// # Example
    /// ```
    /// use dppkf_lib::model::operations::new_game_model::NewGameArgs;
    /// use dppkf_lib::model::types::player::{Player, PlayerType};
    ///
    /// let new_player = Player::from("Grettel".to_string(), PlayerType::AiPlayer);
    /// let mut new_game_args = NewGameArgs::new();
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
    /// use dppkf_lib::model::operations::new_game_model::NewGameArgs;
    /// use doppelkopf_cards_lib::suits::SuitType;
    ///
    /// let mut new_game_args = NewGameArgs::new();
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
    /// use dppkf_lib::model::operations::new_game_model::NewGameArgs;
    /// use doppelkopf_cards_lib::deck::PackSize;
    ///
    /// let mut new_game_args = NewGameArgs::new();
    /// new_game_args.set_pack_size(PackSize::FortyEight);
    ///
    /// assert_eq!(new_game_args.pack_size, PackSize::FortyEight);
    /// ```
    pub fn set_pack_size(&mut self, pack_size: PackSize)  {
        self.pack_size = pack_size;
    }
}