pub fn create_room(settings: &bummer::Settings, room_id: &str) {
    let client = reqwest::blocking::Client::new();
    let auth_key = bummer::get_auth_key().expect("failed to read \"auth.key\"");
    let address = format!(
        "http://{}:{}/createRoom",
        settings.http.address, settings.http.port
    );
    let body = format!(r#"{{ "auth_key": "{auth_key}", "room_id": "{room_id}" }}"#);
    let create = client
        .post(address)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .unwrap();
    assert!(create.status().is_success() && &create.text().unwrap() == "Room created.");
}

pub fn check_room(settings: &bummer::Settings, room_id: &str) -> bool {
    let client = reqwest::blocking::Client::new();
    let auth_key = bummer::get_auth_key().expect("failed to read \"auth.key\"");
    let address = format!(
        "http://{}:{}/checkRoom",
        settings.http.address, settings.http.port
    );
    let body = format!(r#"{{ "auth_key": "{auth_key}", "room_id": "{room_id}" }}"#);
    let create = client
        .get(address)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .unwrap();
    create.status().is_success() && &create.text().unwrap() == "Room exists."
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct PlayerState {
    pub position: (f32, f32),
}
impl Default for PlayerState {
    fn default() -> Self {
        Self {
            position: (0.0, 0.0),
        }
    }
}

pub fn udp(
    settings: &bummer::Settings,
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
    let message = serde_json::to_string(&player_message).unwrap();
    let address = format!("{}:{}", settings.udp.address, settings.udp.port);
    let _ = socket.send_to(message.as_bytes(), &address).unwrap();
    let mut buf = [0; 1024];
    let (amt, _) = socket.recv_from(&mut buf).unwrap();
    String::from_utf8_lossy(&buf[..amt]).to_string()
}
