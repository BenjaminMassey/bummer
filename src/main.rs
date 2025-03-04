mod http;
mod udp;

const ADDRESS: &str = "127.0.0.1";
const HTTP_PORT: u32 = 8080;
const UDP_PORT: u32 = 8081;

fn main() {
    let secret_key = "testeroonie".to_owned(); // TODO: generated, some real kind of key
    let _ = std::fs::write("secret.key", &secret_key);
    let _server = std::thread::spawn(move || {
        let run = udp::server::start(&secret_key);
        if let Err(e) = run {
            println!("Error in server: {e}");
        } else {
            println!("Server done.");
        }
    });
    let _client = std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(10));
        let msg = udp::client::test_message(
            "42069",
            "bob",
            udp::data::PlayerState {
                alive: true,
                position: (1., 2., 3.),
                rotation: (4., 5., 6.),
            },
        );
        if let Err(e) = msg {
            println!("Error in client: {e}");
        } else {
            println!("Client done.");
        }
    });
    http::server::start();
}