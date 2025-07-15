pub fn create_room() {
    let client = reqwest::blocking::Client::new();
    let auth_key = std::fs::read_to_string("auth.key").unwrap();
    let body = format!(r#"{{ "auth_key": "{auth_key}", "room_id": "test" }}"#);
    let create = client.post("http://127.0.0.1:8080/createRoom")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .unwrap();
    assert!(create.status().is_success() && &create.text().unwrap() == "Room created.");
}

pub fn check_room() {
    let client = reqwest::blocking::Client::new();
    let auth_key = std::fs::read_to_string("auth.key").unwrap();
    let body = format!(r#"{{ "auth_key": "{auth_key}", "room_id": "test" }}"#);
    let create = client.get("http://127.0.0.1:8080/checkRoom")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .unwrap();
    assert!(create.status().is_success() && &create.text().unwrap() == "Room exists.");
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct PlayerState {
    pub position: (f32, f32)
}
impl Default for PlayerState {
    fn default() -> Self {
        Self { position: (0.0, 0.0) }
    }
}

pub fn udp(socket: &std::net::UdpSocket, name: &str, x: f32, y: f32) -> String {
    let player_message = bummer::udp::data::PlayerMessage {
        room_id: "test".to_owned(),
        player_id: name.to_owned(),
        state: PlayerState { position: (x, y) },
    };
    let tagged_message = bummer::udp::data::TaggedMessage {
        tag: "player_message".to_owned(),
        data: serde_json::to_string(&player_message).unwrap()
    };
    let message = serde_json::to_string(&tagged_message).unwrap();
    let _ = socket.send_to(
        message.as_bytes(),
        "127.0.0.1:8081",
    ).unwrap();
    let mut buf = [0; 1024];
    let (amt, _) = socket.recv_from(&mut buf).unwrap();
    String::from_utf8_lossy(&buf[..amt]).to_string()
}