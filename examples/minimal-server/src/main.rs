#[derive(serde::Deserialize, serde::Serialize, Clone)]
struct PlayerState;

fn main() {
    bummer::start(PlayerState);
}
