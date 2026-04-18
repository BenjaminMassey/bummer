pub async fn ping() -> &'static str {
    "pong"
}

pub async fn create_room(
    axum::extract::State(state): axum::extract::State<
        std::sync::Arc<crate::http::server::AxumState>,
    >,
    axum::extract::Json(payload): axum::extract::Json<crate::http::data::CreateRoom>,
) -> impl axum::response::IntoResponse {
    let (auth_status, auth_message) = check_auth(&payload.auth_key.clone());
    if auth_status != axum::http::StatusCode::OK {
        return (auth_status, auth_message);
    }
    let room_id = payload.room_id.clone();
    println!("create room a");
    state
        .mpsc_sender
        .send(format!("create_room:{}", &room_id))
        .expect("Error with create_room mpsc send.");
    println!("create room b");
    // TODO: get real response from udp server
    return (axum::http::StatusCode::OK, "Done.".to_owned());
}

pub async fn check_room(
    axum::extract::State(state): axum::extract::State<
        std::sync::Arc<crate::http::server::AxumState>,
    >,
    axum::extract::Json(payload): axum::extract::Json<crate::http::data::CheckRoom>,
) -> impl axum::response::IntoResponse {
    let (auth_status, auth_message) = check_auth(&payload.auth_key.clone());
    if auth_status != axum::http::StatusCode::OK {
        return (auth_status, auth_message);
    }
    let room_id = payload.room_id.clone();
    state
        .mpsc_sender
        .send(format!("check_room:{}", &room_id))
        .expect("Error with create_room mpsc send.");
    // TODO: get real response from udp server
    return (axum::http::StatusCode::OK, "Done.".to_owned());
}

// TODO: some room+players delete system

fn check_auth(key: &str) -> (axum::http::StatusCode, String) {
    let auth_key = crate::get_auth_key();
    if let Some(auth_key) = auth_key {
        if key != &auth_key {
            return (
                axum::http::StatusCode::UNAUTHORIZED,
                "Authorization Failure.".to_owned(),
            );
        }
    } else {
        return (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Error reading \"auth.key\".".to_owned(),
        );
    }
    (
        axum::http::StatusCode::OK,
        "Authorization Success.".to_owned(),
    )
}
