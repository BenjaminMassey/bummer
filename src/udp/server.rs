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
        
        let mut response = "Failed to understand data.".to_owned();
        
        let player_message: Result<crate::udp::data::PlayerMessage, serde_json::Error> = 
            serde_json::from_str(&data);
        if let Ok(msg) = player_message {
            response = handle_player_message(&mut states, msg);
        }

        let create_room_message: Result<crate::udp::data::CreateRoomMessage, serde_json::Error> = 
            serde_json::from_str(&data);
        if let Ok(msg) = create_room_message {
            response = handle_create_room_message(&mut states, secret_key, msg);
        }
        
        socket.send_to(&response.as_bytes(), src)?;
    }
}

fn handle_player_message(
    states: &mut HashMap<String, crate::udp::data::GameState>,
    player_message: crate::udp::data::PlayerMessage
) -> String {
    if let Some(state) = states.get_mut(&player_message.game_id) {
        if let Some(entry) = state.data.get_mut(&player_message.player_id) {
            *entry = player_message.state;
        } else {
            state.data.insert(player_message.player_id, player_message.state);
        }
        let game_msg = crate::udp::data::GameMessage {
            time: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
            state: states[&player_message.game_id].clone(),
        };
        if let Ok(str) = serde_json::to_string(&game_msg) {
            return str;
        } else {
            return "Failure to JSONify game message.".to_owned();
        }
    } else {
        return format!("Game {} does not exist.", &player_message.game_id);
    }
}

fn handle_create_room_message(
    states: &mut HashMap<String, crate::udp::data::GameState>,
    secret_key: &str,
    create_room_message: crate::udp::data::CreateRoomMessage,
) -> String {
    if &create_room_message.secret_key == &secret_key {
        let room_id = &create_room_message.room_id;
        states.insert(
            room_id.clone(),
            crate::udp::data::GameState { data: HashMap::new() },
        );
        return format!("Success in creating room {room_id}");
    }
    "Create room met with incorrect secret key.".to_owned()
}