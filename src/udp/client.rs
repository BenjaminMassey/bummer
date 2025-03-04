// TODO: genericize, lots of copy and paste currently

pub fn test_message(
    game_id: &str,
    player_id: &str,
    state: crate::udp::data::PlayerState,
) -> std::io::Result<()> {
    // Create a UDP socket for the client
    let socket = std::net::UdpSocket::bind(&format!("{}:0", crate::ADDRESS))?; // Bind to any available port
    println!("Client is running on {}", socket.local_addr()?);

    // Server address
    let server_addr = format!("{}:{}", crate::ADDRESS, crate::UDP_PORT);

    // Create a game message
    let message = crate::udp::data::PlayerMessage {
        game_id: game_id.to_owned(),
        player_id: player_id.to_owned(),
        state: state,
    };

    // Serialize the message to bytes
    let message_bytes = bincode::serialize(&message).unwrap();

    // Send the serialized message to the server
    socket.send_to(&message_bytes, server_addr)?;
    println!("Sent to server: {:?}", message);

    // Receive a response from the server
    let mut buf = [0; 1024];
    let (amt, _) = socket.recv_from(&mut buf)?;

    // Deserialize the response bytes into a GameMessage
    let response_message: Result<crate::udp::data::GameMessage, _> = bincode::deserialize(&buf[..amt]);
    if let Ok(res) = response_message {
        println!("Received GameMessage from server: {:?}", res);
    } else {
        let response_message: Result<String, _> = bincode::deserialize(&buf[..amt]);
        if let Ok(res) = response_message {
            println!("Got direct message from server: {}", res);
        } else {
            println!("No message from server, or failed to parse.");
        }
    }
    
    Ok(())
}

pub fn create_room(
    room_id: &str,
    secret_key: &str,
) -> std::io::Result<()> {
    let socket = std::net::UdpSocket::bind(&format!("{}:0", crate::ADDRESS))?; 
    println!("Client is running on {}", socket.local_addr()?);

    let server_addr = format!("{}:{}", crate::ADDRESS, crate::UDP_PORT);

    let message = crate::udp::data::CreateRoomMessage {
        room_id: room_id.to_owned(),
        secret_key: secret_key.to_owned(),
    };
    let message_bytes = bincode::serialize(&message).unwrap();

    socket.send_to(&message_bytes, server_addr)?;
    println!("Sent to server: {:?}", message);

    let mut buf = [0; 1024];
    let (amt, _) = socket.recv_from(&mut buf)?;

    let response_message: Result<crate::udp::data::CreateRoomMessage, _> = bincode::deserialize(&buf[..amt]);
    if let Ok(res) = response_message {
        println!("Received CreateRoomMessage from server: {:?}", res);
    } else {
        let response_message: Result<String, _> = bincode::deserialize(&buf[..amt]);
        if let Ok(res) = response_message {
            println!("Got direct message from server: {}", res);
        } else {
            println!("No message from server, or failed to parse.");
        }
    }

    Ok(())
}