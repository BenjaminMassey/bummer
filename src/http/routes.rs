pub async fn ping() -> &'static str {
    "pong"
}

pub async fn create_room(
    axum::extract::Json(payload): axum::extract::Json<crate::http::data::CreateRoom>,
) -> impl axum::response::IntoResponse {
    // Check that from a valid client
    let auth = std::fs::read_to_string("auth.key");
    if let Err(e) = auth {
        println!("Auth key read error: {e}");
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "failed with internal auth key reading");
    } else {
        let auth_key = auth.unwrap();
        if &payload.auth_key != &auth_key {
            return (axum::http::StatusCode::UNAUTHORIZED, "auth key failure");
        }
    }

    // Verify that UDP call to create room coming from app itself
    let secret = std::fs::read_to_string("secret.key");
    if let Err(e) = secret {
        println!("Secret key read error: {e}");
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "failed with internal secret key reading");
    } else {
        let secret_key = secret.unwrap();
        let _ = crate::udp::client::create_room(&payload.room_id, &secret_key); // TODO: verify and cases
        return (axum::http::StatusCode::OK, "room creation requested");
    }
}