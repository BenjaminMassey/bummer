#[derive(serde::Deserialize)]
pub struct CreateRoom {
    pub auth_key: String,
    pub room_id: String,
}