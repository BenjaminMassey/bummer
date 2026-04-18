use std::collections::HashMap;

pub fn handle_player_message<T>(
    states: &mut HashMap<String, crate::udp::data::GameState<T>>,
    player_message: crate::udp::data::PlayerMessage<T>,
) -> String
where
    T: serde::Serialize + serde::de::DeserializeOwned + Clone,
{
    if let Some(state) = states.get_mut(&player_message.room_id) {
        state.last_time = crate::util::epoch_time();
        if let Some(entry) = state.data.get_mut(&player_message.player_id) {
            *entry = crate::udp::data::TimedPlayerState {
                index: entry.index,
                last_time: crate::util::epoch_time(),
                state: player_message.state,
            };
        } else {
            state.data.insert(
                player_message.player_id.clone(),
                crate::udp::data::TimedPlayerState {
                    index: state.names.len() as u32,
                    last_time: crate::util::epoch_time(),
                    state: player_message.state,
                },
            );
            state.names.push(player_message.player_id);
        }
        let game_msg = crate::udp::data::GameMessage {
            time: crate::util::epoch_time(),
            state: states[&player_message.room_id].clone(),
        };
        if let Ok(game_str) = serde_json::to_string(&game_msg) {
            return game_str;
        } else {
            return crate::udp::messages::INTERNAL_SERVER_ERROR.to_owned();
        }
    } else {
        return crate::udp::messages::NO_ROOM.to_owned();
    }
}

pub fn create_room<T>(
    states: &mut HashMap<String, crate::udp::data::GameState<T>>,
    room_id: &str,
) -> String
where
    T: serde::Serialize + serde::de::DeserializeOwned + Clone,
{
    if states.contains_key(room_id) {
        return crate::udp::messages::EXISTING_ROOM.to_owned();
    } else {
        states.insert(
            room_id.to_owned(),
            crate::udp::data::GameState {
                names: vec![],
                last_time: crate::util::epoch_time(),
                data: HashMap::new(),
            },
        );
        return crate::udp::messages::ROOM_CREATED.to_owned();
    }
}

pub fn check_room<T>(
    states: &mut HashMap<String, crate::udp::data::GameState<T>>,
    room_id: &str,
) -> String
where
    T: serde::Serialize + serde::de::DeserializeOwned + Clone,
{
    if states.contains_key(room_id) {
        return crate::udp::messages::ROOM_EXISTS.to_owned();
    }
    crate::udp::messages::NO_ROOM.to_owned()
}
