#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct PlayerMessage<T> {
    pub room_id: String,
    pub player_id: String,
    pub state: T,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TimedPlayerState<T> {
    pub index: u32,
    pub last_time: u128,
    pub state: T,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GameMessage<T> {
    pub time: u128,
    pub state: GameState<T>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GameState<T> {
    pub names: Vec<String>,
    pub last_time: u128,
    pub data: std::collections::HashMap<String, TimedPlayerState<T>>,
}
