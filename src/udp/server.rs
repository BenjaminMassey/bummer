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

        let response: Vec<u8> = match bincode::deserialize::<crate::udp::data::IncomingMessage>(&buf[..amt]) {
            Ok(crate::udp::data::IncomingMessage::PlayerMessage(player_message)) => {
                handle_player_message(&mut states, player_message)
            },
            Ok(crate::udp::data::IncomingMessage::CreateRoomMessage(create_room_message)) => {
                handle_create_room_message(&mut states, &secret_key, create_room_message)
            },
            _ => bincode::serialize("Unknown message type.").unwrap(),
        };
        
        socket.send_to(&response, src)?;
    }
}

fn handle_player_message(
    states: &mut HashMap<String, crate::udp::data::GameState>,
    player_message: crate::udp::data::PlayerMessage
) -> Vec<u8> {
    if let Some(state) = states.get_mut(&player_message.game_id) {
        if let Some(entry) = state.data.get_mut(&player_message.player_id) {
            *entry = player_message.state;
        } else {
            state.data.insert(player_message.player_id, player_message.state);
        }
        if let Ok(msg) = bincode::serialize(
            &crate::udp::data::GameMessage {
                time: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
                state: states[&player_message.game_id].clone(),
            }
        )
        {
            return msg;
        }
    } else {
        return bincode::serialize(&format!("Game {} does not exist.", &player_message.game_id)).unwrap();
    }

    bincode::serialize("Generic handling error with player message.").unwrap()
}

fn handle_create_room_message(
    states: &mut HashMap<String, crate::udp::data::GameState>,
    secret_key: &str,
    create_room_message: crate::udp::data::CreateRoomMessage,
) -> Vec<u8> {
    if &create_room_message.secret_key == &secret_key {
        let room_id = &create_room_message.room_id;
        states.insert(
            room_id.clone(),
            crate::udp::data::GameState { data: HashMap::new() },
        );
        return bincode::serialize(&format!("Success in creating room {room_id}")).unwrap();
    }
    bincode::serialize("Create room met with incorrect secret key.").unwrap()
}