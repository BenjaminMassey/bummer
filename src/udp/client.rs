pub fn send_message(message: &str, settings: &crate::settings::Settings) -> std::io::Result<String> {
    // Create a UDP socket for the client
    let socket = std::net::UdpSocket::bind(&format!("{}:0", settings.udp.address))?;
    println!("Client is running on {}", socket.local_addr()?);

    // Server address
    let server_addr = format!("{}:{}", settings.udp.address, settings.udp.port);

    // Send the serialized message to the server
    socket.send_to(message.as_bytes(), server_addr)?;
    //println!("Sent to server: {:?}", message);

    // Receive a response from the server
    let mut buf = [0; 1024];
    let (amt, _) = socket.recv_from(&mut buf)?;

    let data = String::from_utf8_lossy(&buf[..amt]);
    Ok(data.to_string())
}

pub fn create_room(
    room_id: &str,
    secret_key: &str,
    settings: &crate::settings::Settings,
) -> Option<String> {
    let create_room_message = crate::udp::data::CreateRoomMessage {
        room_id: room_id.to_owned(),
        secret_key: secret_key.to_owned(),
    };
    let tagged_message = crate::udp::data::TaggedMessage {
        tag: "create_room".to_owned(),
        data: serde_json::to_string(&create_room_message).unwrap(),
    };
    let response = send_message(&serde_json::to_string(&tagged_message).unwrap(), settings);
    if let Ok(res) = response {
        return Some(res);
    } else {
        return None;
    }
}

pub fn check_room(
    room_id: &str,
    secret_key: &str,
    settings: &crate::settings::Settings,
) -> Option<String> {
    let check_room_message = crate::udp::data::CheckRoomMessage {
        room_id: room_id.to_owned(),
        secret_key: secret_key.to_owned(),
    };
    let tagged_message = crate::udp::data::TaggedMessage {
        tag: "check_room".to_owned(),
        data: serde_json::to_string(&check_room_message).unwrap(),
    };
    let response = send_message(&serde_json::to_string(&tagged_message).unwrap(), settings);
    if let Ok(res) = response {
        return Some(res);
    } else {
        return None;
    }
}

pub fn delete_room(
    room_id: &str,
    secret_key: &str,
    settings: &crate::settings::Settings,
) -> Option<String> {
    let delete_room_message = crate::udp::data::DeleteRoomMessage {
        room_id: room_id.to_owned(),
        secret_key: secret_key.to_owned(),
    };
    let tagged_message = crate::udp::data::TaggedMessage {
        tag: "delete_room".to_owned(),
        data: serde_json::to_string(&delete_room_message).unwrap(),
    };
    let response = send_message(&serde_json::to_string(&tagged_message).unwrap(), settings);
    if let Ok(res) = response {
        return Some(res);
    } else {
        return None;
    }
}

pub fn delete_players(
    room_id: &str,
    secret_key: &str,
    settings: &crate::settings::Settings,
) -> Option<String> {
    let delete_players_message = crate::udp::data::DeletePlayersMessage {
        room_id: room_id.to_owned(),
        secret_key: secret_key.to_owned(),
    };
    let tagged_message = crate::udp::data::TaggedMessage {
        tag: "delete_players".to_owned(),
        data: serde_json::to_string(&delete_players_message).unwrap(),
    };
    let response = send_message(&serde_json::to_string(&tagged_message).unwrap(), settings);
    if let Ok(res) = response {
        return Some(res);
    } else {
        return None;
    }
}