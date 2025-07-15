use std::collections::HashMap;

pub fn start(secret_key: &str) -> std::io::Result<()> {
    let host = format!("{}:{}", crate::ADDRESS, crate::UDP_PORT);
    let socket = std::net::UdpSocket::bind(&host)?;
    println!("UDP server is listening on {host}");

    // Game/Room ID => Game State
    let mut states: HashMap<String, crate::udp::data::GameState> = HashMap::new();

    let mut buf = [0; 1024];

    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;

        let data = String::from_utf8_lossy(&buf[..amt]);
        
        let mut response = "failure".to_owned();
        
        let tagged_message: Result<crate::udp::data::TaggedMessage, serde_json::Error> = 
            serde_json::from_str(&data);
        if let Ok(message) = tagged_message {
            if &message.tag == "player_message" {
                let player_message: Result<crate::udp::data::PlayerMessage, serde_json::Error> = 
                    serde_json::from_str(&message.data);
                if let Ok(msg) = player_message {
                    response = handle_player_message(&mut states, msg);
                }
            } else if &message.tag == "create_room" {
                let create_room_message: Result<crate::udp::data::CreateRoomMessage, serde_json::Error> = 
                    serde_json::from_str(&message.data);
                if let Ok(msg) = create_room_message {
                    response = handle_create_room_message(&mut states, secret_key, msg);
                }
            } else if &message.tag == "check_room" {
                let check_room_message: Result<crate::udp::data::CheckRoomMessage, serde_json::Error> = 
                    serde_json::from_str(&message.data);
                if let Ok(msg) = check_room_message {
                    response = handle_check_room_message(&mut states, secret_key, msg);
                }
            } else if &message.tag == "delete_room" {
                let delete_room_message: Result<crate::udp::data::DeleteRoomMessage, serde_json::Error> = 
                    serde_json::from_str(&message.data);
                if let Ok(msg) = delete_room_message {
                    response = handle_delete_room_message(&mut states, secret_key, msg);
                }
            } else if &message.tag == "delete_players" {
                let delete_players_message: Result<crate::udp::data::DeletePlayersMessage, serde_json::Error> = 
                    serde_json::from_str(&message.data);
                if let Ok(msg) = delete_players_message {
                    response = handle_delete_players_message(&mut states, secret_key, msg);
                }
            }
            socket.send_to(&response.as_bytes(), src)?;
        } else if let Err(e) = tagged_message {
            socket.send_to(&format!("{e:?}").as_bytes(), src)?;
        }
    }
}

fn handle_player_message(
    states: &mut HashMap<String, crate::udp::data::GameState>,
    player_message: crate::udp::data::PlayerMessage
) -> String {
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
            let tagged_msg = crate::udp::data::TaggedMessage {
                tag: "game_state".to_owned(),
                data: game_str,
            };
            if let Ok(str) = serde_json::to_string(&tagged_msg) {
                return str;
            } else {
                return r#"{ "tag": "error", "data": "parse error 2" }"#.to_owned();
            }
        } else {
            return r#"{ "tag": "error", "data": "parse error 1" }"#.to_owned();
        }
    } else {
        return format!(
            r#"{{ "tag": "error", "data": "{}" }}"#,
            crate::udp::messages::NO_ROOM,
        );
    }
}

fn handle_create_room_message(
    states: &mut HashMap<String, crate::udp::data::GameState>,
    secret_key: &str,
    create_room_message: crate::udp::data::CreateRoomMessage,
) -> String {
    if &create_room_message.secret_key == &secret_key {
        let room_id = &create_room_message.room_id;
        if states.contains_key(room_id) {
            return format!(
                r#"{{ "tag": "error", "data": "{}" }}"#,
                crate::udp::messages::EXISTING_ROOM,
            );
        } else {
            states.insert(
                room_id.to_owned(),
                crate::udp::data::GameState {
                    names: vec![],
                    last_time: crate::util::epoch_time(),
                    data: HashMap::new(),
                },
            );
            return r#"{ "tag": "success", "data": "Room created." }"#.to_owned();
        }
    }
    r#"{ "tag": "error", "data": "Internal Server Error 5" }"#.to_owned()
}

fn handle_check_room_message(
    states: &mut HashMap<String, crate::udp::data::GameState>,
    secret_key: &str,
    check_room_message: crate::udp::data::CheckRoomMessage,
) -> String {
    if &check_room_message.secret_key == &secret_key {
        let room_id = &check_room_message.room_id;
        if states.contains_key(room_id) {
            return r#"{ "tag": "success", "data": "Room exists." }"#.to_owned();
        } else {
            return format!(
                r#"{{ "tag": "success", "data": "{}" }}"#,
                crate::udp::messages::NO_ROOM,
            );
        }
    }
    r#"{ "tag": "error", "data": "Internal Server Error 6" }"#.to_owned()
}

fn handle_delete_room_message(
    states: &mut HashMap<String, crate::udp::data::GameState>,
    secret_key: &str,
    delete_room_message: crate::udp::data::DeleteRoomMessage,
) -> String {
    if &delete_room_message.secret_key == &secret_key {
        let room_id = &delete_room_message.room_id;
        if states.contains_key(room_id) {
            if crate::util::epoch_time() - states[room_id].last_time >= 4000 {
                let _ = states.remove(room_id);
                return format!(
                    r#"{{ "tag": "success", "data": "{}" }}"#,
                    crate::udp::messages::ROOM_EXPIRED,
                );
            } else {
                return r#"{ "tag": "success", "data": "Room is active." }"#.to_owned();
            }
        } else {
            return format!(
                r#"{{ "tag": "error", "data": "{}" }}"#,
                crate::udp::messages::NO_ROOM,
            );
        }
    }
    r#"{ "tag": "error", "data": "Internal Server Error 7" }"#.to_owned()
}

fn handle_delete_players_message(
    states: &mut HashMap<String, crate::udp::data::GameState>,
    secret_key: &str,
    delete_players_message: crate::udp::data::DeletePlayersMessage,
) -> String {
    if &delete_players_message.secret_key == &secret_key {
        let room_id = &delete_players_message.room_id;
        if let Some(state) = states.get_mut(room_id) {
            let mut names: Vec<String> = vec![];
            for name in &state.names {
                if state.data.contains_key(name) {
                    if crate::util::epoch_time() - state.data[name].last_time >= 2000 {
                        state.data.remove(name);
                        println!("Deleted player {name} from room {room_id}.");
                    } else {
                        names.push(name.clone());
                    }
                }
            }
            state.names = names;
            return r#"{ "tag": "success", "data": "Delete players handled." }"#.to_owned();
        } else {
            return format!(
                r#"{{ "tag": "error", "data": "{}" }}"#,
                crate::udp::messages::NO_ROOM,
            );
        }
    }
    r#"{ "tag": "error", "data": "Internal Server Error 8" }"#.to_owned()
}