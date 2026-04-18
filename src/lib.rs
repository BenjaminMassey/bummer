mod http;
mod settings;
pub mod udp;
mod util;

pub use self::udp::data::GameState;
pub use self::udp::data::PlayerMessage;

pub fn start<T>(state_example: T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + Clone + std::marker::Send + 'static,
{
    let (send_to_http, receive_from_udp) = std::sync::mpsc::channel::<String>();
    let (send_to_udp, receive_from_http) = std::sync::mpsc::channel::<String>();
    let _udp = std::thread::spawn(move || {
        let run = udp::server::start(send_to_http, receive_from_http, state_example);
        if let Err(e) = run {
            println!("Error in server: {e}");
        } else {
            println!("Server done.");
        }
    });
    http::server::start(receive_from_udp, send_to_udp);
}

pub fn get_auth_key() -> Option<String> {
    if let Ok(auth_key) = std::fs::read_to_string("auth.key") {
        return Some(auth_key.trim().to_owned());
    }
    None
}
