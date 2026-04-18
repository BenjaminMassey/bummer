use std::collections::HashMap;

pub fn start<T>(
    send_to_http: std::sync::mpsc::Sender<String>,
    receive_from_http: std::sync::mpsc::Receiver<String>,
    state_example: T,
) -> std::io::Result<()>
where
    T: serde::Serialize + serde::de::DeserializeOwned + Clone,
{
    let settings = crate::settings::get_settings();

    let host = format!("{}:{}", settings.udp.address, settings.udp.port);
    let socket = std::net::UdpSocket::bind(&host)?;
    socket
        .set_nonblocking(true)
        .expect("Failed to set UDP socket to nonblocking.");
    println!("UDP server is listening on {host}");

    // Game/Room ID => Game State
    let mut states: HashMap<String, crate::udp::data::GameState<T>> = HashMap::new();

    let mut buf = [0; 1024];

    let mut cull_time = crate::util::epoch_time();

    loop {
        if let Ok((amt, src)) = socket.recv_from(&mut buf) {
            let data = String::from_utf8_lossy(&buf[..amt]);

            let mut response = crate::udp::messages::INTERNAL_SERVER_ERROR.to_owned();

            let player_message: Result<crate::udp::data::PlayerMessage<T>, serde_json::Error> =
                serde_json::from_str(&data);
            if let Ok(msg) = player_message {
                response = crate::udp::actions::handle_player_message(&mut states, msg);
            }
            socket.send_to(&response.as_bytes(), src)?;
        }

        if let Ok(http_msg) = receive_from_http.try_recv() {
            handle_http_message(
                send_to_http.clone(),
                state_example.clone(),
                &mut states,
                &http_msg,
            );
        }

        if (crate::util::epoch_time() - cull_time) >= 1000 {
            crate::udp::guillotine::delete_stale_rooms(&mut states);
            crate::udp::guillotine::delete_stale_players(&mut states);
            cull_time = crate::util::epoch_time();
        }
    }
}

fn handle_http_message<T>(
    send_to_http: std::sync::mpsc::Sender<String>,
    _state_example: T,
    states: &mut HashMap<String, crate::udp::data::GameState<T>>,
    message: &str,
) where
    T: serde::Serialize + serde::de::DeserializeOwned + Clone,
{
    let pieces: Vec<&str> = message.split(":").collect();
    assert!(pieces.len() == 2);
    let tag = pieces[0].to_owned();
    let data = pieces[1].to_owned();
    let mut response = crate::udp::messages::INTERNAL_SERVER_ERROR.to_owned();
    if tag == "create_room" {
        response = crate::udp::actions::create_room(states, &data);
    } else if tag == "check_room" {
        response = crate::udp::actions::check_room(states, &data);
    }
    send_to_http
        .send(response)
        .expect("Error sending mspc to http.");
}
