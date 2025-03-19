#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TaggedMessage {
    pub tag: String,
    pub data: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PlayerMessage {
    pub room_id: String,
    pub player_id: String,
    pub state: PlayerState,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TimedPlayerState {
    pub index: u32,
    pub last_time: u128,
    pub state: PlayerState,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PlayerState {
    pub alive: bool,
    pub ready: bool,
    pub position: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CreateRoomMessage {
    pub room_id: String,
    pub secret_key: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CheckRoomMessage {
    pub room_id: String,
    pub secret_key: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct DeleteRoomMessage {
    pub room_id: String,
    pub secret_key: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct DeletePlayersMessage {
    pub room_id: String,
    pub secret_key: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GameMessage {
    pub time: u128,
    pub state: GameState,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GameState {
    pub names: Vec<String>,
    pub last_time: u128,
    pub data: std::collections::HashMap<String, TimedPlayerState>,
}