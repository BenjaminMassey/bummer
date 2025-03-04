pub async fn ping() -> &'static str {
    "pong"
}

pub async fn create_room( // TODO: authentication that this is some legit client
    axum::extract::Json(payload): axum::extract::Json<crate::http::data::CreateRoom>,
) -> &'static str { // TODO: real response
    let read = std::fs::read_to_string("secret.key");
    if let Err(e) = read {
        println!("Error with reading secret key : {e}.");
    } else {
        let key = read.unwrap();
        let _ = crate::udp::client::create_room(&payload.room_id, &key);
    }
    "ok"
}