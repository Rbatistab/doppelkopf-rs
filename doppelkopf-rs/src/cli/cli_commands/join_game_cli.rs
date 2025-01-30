use log::debug;

pub fn join_game(game_id: &Option<String>, player_name: &Option<String>) {
    debug!("Joining game....");
    debug!("Provided game_id: {:?}", game_id);
    debug!("Provided player_name: {:?}", player_name);
}