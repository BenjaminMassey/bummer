use std::collections::HashMap;

pub fn delete_stale_rooms<T>(states: &mut HashMap<String, crate::udp::data::GameState<T>>) {
    let mut to_delete: Vec<String> = vec![];
    for room_id in states.keys() {
        if crate::util::epoch_time() - states[room_id].last_time >= 2000 {
            to_delete.push(room_id.to_owned());
        }
    }
    for room_id in to_delete {
        let _ = states.remove(&room_id);
    }
}

pub fn delete_stale_players<T>(states: &mut HashMap<String, crate::udp::data::GameState<T>>) {
    let room_ids: Vec<String> = states.keys().cloned().collect();
    for room_id in room_ids {
        if let Some(state) = states.get_mut(&room_id) {
            let mut names: Vec<String> = vec![];
            for name in &state.names {
                if state.data.contains_key(name) {
                    if crate::util::epoch_time() - state.data[name].last_time >= 1000 {
                        state.data.remove(name);
                    } else {
                        names.push(name.clone());
                    }
                }
            }
            state.names = names;
        }
    }
}
