use std::collections::HashMap;

pub fn start<T>(
    mpsc_receiver: std::sync::mpsc::Receiver<String>,
    state_example: T,
) -> std::io::Result<()>
where
    T: serde::Serialize + serde::de::DeserializeOwned + Clone,
{
    let settings = crate::settings::get_settings();

    let host = format!("{}:{}", settings.udp.address, settings.udp.port);
    let socket = std::net::UdpSocket::bind(&host)?;
    println!("UDP server is listening on {host}");

    // Game/Room ID => Game State
    let mut states: HashMap<String, crate::udp::data::GameState<T>> = HashMap::new();

    let mut buf = [0; 1024];

    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;

        let data = String::from_utf8_lossy(&buf[..amt]);

        let mut response = crate::udp::messages::INTERNAL_SERVER_ERROR.to_owned();

        let player_message: Result<crate::udp::data::PlayerMessage<T>, serde_json::Error> =
            serde_json::from_str(&data);
        if let Ok(msg) = player_message {
            response = crate::udp::actions::handle_player_message(&mut states, msg);
        }
        println!("udp (out): {}", &response);
        socket.send_to(&response.as_bytes(), src)?;

        if let Ok(mpsc_msg) = mpsc_receiver.try_recv() {
            println!("mpsc (in): {}", mpsc_msg);
            handle_mpsc_message(state_example.clone(), &mut states, &mpsc_msg);
        }
    }
}

fn handle_mpsc_message<T>(
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
    } else if tag == "delete_room" {
        response = crate::udp::actions::delete_room(states, &data);
    } else if tag == "delete_players" {
        response = crate::udp::actions::delete_players(states, &data);
    }
    println!("mpsc (out): {}", &response);
    // TODO: send this out such that http can respond
}
