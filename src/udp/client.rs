pub fn send_message(message: crate::udp::data::IncomingMessage) -> std::io::Result<Vec<u8>> {
    // Create a UDP socket for the client
    let socket = std::net::UdpSocket::bind(&format!("{}:0", crate::ADDRESS))?; // Bind to any available port
    println!("Client is running on {}", socket.local_addr()?);

    // Server address
    let server_addr = format!("{}:{}", crate::ADDRESS, crate::UDP_PORT);

    // Serialize the message to bytes
    let message_bytes = bincode::serialize(&message).unwrap();

    // Send the serialized message to the server
    socket.send_to(&message_bytes, server_addr)?;
    println!("Sent to server: {:?}", message);

    // Receive a response from the server
    let mut buf = [0; 1024];
    let (amt, _) = socket.recv_from(&mut buf)?;

    // Deserialize the response bytes into a GameMessage
    Ok(buf[..amt].to_vec())
}

pub fn _test_message(
    game_id: &str,
    player_id: &str,
    state: crate::udp::data::PlayerState,
) {
    let player_message = crate::udp::data::PlayerMessage {
        game_id: game_id.to_owned(),
        player_id: player_id.to_owned(),
        state,
    };
    let response = send_message(
        crate::udp::data::IncomingMessage::PlayerMessage(player_message)
    );
    if let Ok(res) = response {
        let response_message: Result<crate::udp::data::GameMessage, _> = bincode::deserialize(&res);
        if let Ok(msg) = response_message {
            println!("Received GameMessage from server: {:?}", msg);
        } else {
            let response_message: Result<String, _> = bincode::deserialize(&res);
            if let Ok(msg) = response_message {
                println!("Got direct message from server: {}", msg);
            } else {
                println!("No message from server, or failed to parse.");
            }
        }
    } else {
        println!("No message from server, or failed to parse.");
    }
}

pub fn create_room(
    room_id: &str,
    secret_key: &str,
) {
    let create_room_message = crate::udp::data::CreateRoomMessage {
        room_id: room_id.to_owned(),
        secret_key: secret_key.to_owned(),
    };
    let response = send_message(
        crate::udp::data::IncomingMessage::CreateRoomMessage(create_room_message)
    );
    if let Ok(res) = response {
        let response_message: Result<crate::udp::data::CreateRoomMessage, _> = bincode::deserialize(&res);
        if let Ok(msg) = response_message {
            println!("Received CreateRoomMessage from server: {:?}", msg);
        } else {
            let response_message: Result<String, _> = bincode::deserialize(&res);
            if let Ok(msg) = response_message {
                println!("Got direct message from server: {}", msg);
            } else {
                println!("No message from server, or failed to parse.");
            }
        }
    }
}