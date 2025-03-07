#[derive(serde::Deserialize)]
pub struct CreateRoom {
    pub auth_key: String,
    pub room_id: String,
}

#[derive(serde::Deserialize)]
pub struct CheckRoom {
    pub auth_key: String,
    pub room_id: String,
}