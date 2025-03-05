pub fn send_message(message: &str) -> std::io::Result<String> {
    // Create a UDP socket for the client
    let socket = std::net::UdpSocket::bind(&format!("{}:0", crate::ADDRESS))?; // Bind to any available port
    println!("Client is running on {}", socket.local_addr()?);

    // Server address
    let server_addr = format!("{}:{}", crate::ADDRESS, crate::UDP_PORT);

    // Send the serialized message to the server
    socket.send_to(message.as_bytes(), server_addr)?;
    println!("Sent to server: {:?}", message);

    // Receive a response from the server
    let mut buf = [0; 1024];
    let (amt, _) = socket.recv_from(&mut buf)?;

    let data = String::from_utf8_lossy(&buf[..amt]);
    Ok(data.to_string())
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
    let response = send_message(&serde_json::to_string(&player_message).unwrap());
    if let Ok(res) = response {
        println!("Received GameMessage from server: {res}");
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
    let response = send_message(&serde_json::to_string(&create_room_message).unwrap());
    if let Ok(res) = response {
        println!("Received GameMessage from server: {res}");
    } else {
        println!("No message from server, or failed to parse.");
    }
}