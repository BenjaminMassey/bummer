use rand::Rng;

mod http;
mod udp;
mod util;

const ADDRESS: &str = "127.0.0.1";
const HTTP_PORT: u32 = 8080;
const UDP_PORT: u32 = 8081;

fn main() {
    let secret_key: String = rand::rng()
        .sample_iter(rand::distr::Alphanumeric)
        .take(512)
        .map(char::from)
        .collect();
    let _ = std::fs::write("secret.key", &secret_key);
    let _server = std::thread::spawn(move || {
        let run = udp::server::start(&secret_key);
        if let Err(e) = run {
            println!("Error in server: {e}");
        } else {
            println!("Server done.");
        }
    });
    http::server::start();
}