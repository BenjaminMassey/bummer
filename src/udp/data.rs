#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum IncomingMessage {
    PlayerMessage(PlayerMessage),
    CreateRoomMessage(CreateRoomMessage),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PlayerMessage {
    pub game_id: String,
    pub player_id: String,
    pub state: PlayerState,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PlayerState {
    pub alive: bool,
    pub position: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CreateRoomMessage {
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
    pub data: std::collections::HashMap<String, PlayerState>,
}