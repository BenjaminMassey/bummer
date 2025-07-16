use rand::Rng;

mod http;
mod settings;
pub mod udp;
mod util;

pub use self::udp::data::TaggedMessage;
pub use self::udp::data::PlayerMessage;
pub use self::udp::data::GameState;

pub fn start<T>(state_example: T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + Clone + std::marker::Send + 'static
{
    let secret_key: String = rand::rng()
        .sample_iter(rand::distr::Alphanumeric)
        .take(512)
        .map(char::from)
        .collect();
    let _ = std::fs::write("secret.key", &secret_key);
    let _udp = std::thread::spawn(move || {
        let run = udp::server::start(&secret_key, state_example);
        if let Err(e) = run {
            println!("Error in server: {e}");
        } else {
            println!("Server done.");
        }
    });
    http::server::start();
}