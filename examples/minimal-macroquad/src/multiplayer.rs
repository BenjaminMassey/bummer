pub fn create_room(server: &str) {
    let client = reqwest::blocking::Client::new();
    let auth_key = std::fs::read_to_string("auth.key").unwrap();
    let auth_key = auth_key.trim_end();
    let body = format!(r#"{{ "auth_key": "{auth_key}", "room_id": "test" }}"#);
    let server_url = format!("http://{server}:8080/createRoom");
    let create = client.post(&server_url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .unwrap();
    eprintln!("{create:?}");
    assert!(create.status().is_success() && &create.text().unwrap() == "Room created.");
}

pub fn check_room(server: &str) {
    let client = reqwest::blocking::Client::new();
    let auth_key = std::fs::read_to_string("auth.key").unwrap();
    let body = format!(r#"{{ "auth_key": "{auth_key}", "room_id": "test" }}"#);
    let server_url = format!("http://{server}:8080/checkRoom");
    let create = client.get(&server_url)
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

pub fn udp(
    socket: &std::net::UdpSocket,
    name: &str,
    x: f32,
    y: f32,
) -> String {
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
    let _ = socket.send(message.as_bytes()).unwrap();
    let mut buf = [0; 1024];
    let amt = socket.recv(&mut buf).unwrap();
    String::from_utf8_lossy(&buf[..amt]).to_string()
}
