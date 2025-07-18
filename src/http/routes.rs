pub async fn ping() -> &'static str {
    "pong"
}

pub async fn create_room(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::http::server::AxumState>>,
    axum::extract::Json(payload): axum::extract::Json<crate::http::data::CreateRoom>,
) -> impl axum::response::IntoResponse {
    // Check that from a valid client
    let auth = std::fs::read_to_string("auth.key");
    if let Err(e) = auth {
        println!("Auth key read error: {e}");
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error 1.".to_owned());
    } else {
        let auth_key = auth.unwrap();
        if payload.auth_key != auth_key {
            return (axum::http::StatusCode::UNAUTHORIZED, "Authorization Failure.".to_owned());
        }
    }

    // Verify that UDP call to create room coming from app itself
    let secret = std::fs::read_to_string("secret.key");
    if let Err(e) = secret {
        println!("Secret key read error: {e}");
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error 2.".to_owned())
    } else {
        let room_id = payload.room_id.clone();
        let secret_key = secret.unwrap();
        let response = crate::udp::client::create_room(&room_id, &secret_key, &state.settings);
        let mut code = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
        let mut message = "Failed request.".to_owned();
        if let Some(res) = response {
            let tagged_msg: Result<crate::udp::data::TaggedMessage, _> = serde_json::from_str(&res);
            if let Ok(msg) = tagged_msg {
                if &msg.tag == "success" {
                    code = axum::http::StatusCode::OK;
                } else if &msg.tag == "error" && msg.data == crate::udp::messages::EXISTING_ROOM {
                    code = axum::http::StatusCode::BAD_REQUEST;
                }
                message = msg.data;
            }
        }
        if code == axum::http::StatusCode::OK {
            let settings_clone = state.settings.clone();
            let _handler = std::thread::spawn(move || {
                handle_room_state(&room_id, &secret_key, &settings_clone);
            });
        }
        (code, message.to_owned())
    }
}

pub async fn check_room(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::http::server::AxumState>>,
    axum::extract::Json(payload): axum::extract::Json<crate::http::data::CheckRoom>,
) -> impl axum::response::IntoResponse {
    // Check that from a valid client
    let auth = std::fs::read_to_string("auth.key");
    if let Err(e) = auth {
        println!("Auth key read error: {e}");
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error 3.".to_owned());
    } else {
        let auth_key = auth.unwrap();
        if payload.auth_key != auth_key {
            return (axum::http::StatusCode::UNAUTHORIZED, "Authorization Failure.".to_owned());
        }
    }

    // Verify that UDP call to create room coming from app itself
    let secret = std::fs::read_to_string("secret.key");
    if let Err(e) = secret {
        println!("Secret key read error: {e}");
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error 4.".to_owned())
    } else {
        let secret_key = secret.unwrap();
        let response = crate::udp::client::check_room(&payload.room_id, &secret_key, &state.settings);
        let mut code = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
        let mut message = "Failed request.".to_owned();
        if let Some(res) = response {
            let tagged_msg: Result<crate::udp::data::TaggedMessage, _> = serde_json::from_str(&res);
            if let Ok(msg) = tagged_msg {
                if &msg.tag == "success" {
                    code = axum::http::StatusCode::OK;
                    message = msg.data;
                }
            }
        }
        (code, message.to_owned())
    }
}

fn handle_room_state(
    room_id: &str,
    secret_key: &str,
    settings: &crate::settings::Settings,
) {
    //println!("Starting handler for room {room_id}.");
    loop {
        let response = crate::udp::client::delete_room(room_id, secret_key, settings);
        if let Some(res) = response {
            let tagged_msg: Result<crate::udp::data::TaggedMessage, _> = serde_json::from_str(&res);
            if let Ok(msg) = tagged_msg {
                if &msg.tag == "success" && msg.data == crate::udp::messages::ROOM_EXPIRED {
                    break;
                }
                if &msg.tag == "error" && msg.data == crate::udp::messages::NO_ROOM {
                    println!("internal error of http::routes::handle_room_state(..) trying room that doesn't exist");
                    break;
                }
            }
        }
        let _ = crate::udp::client::delete_players(room_id, secret_key, settings);
        //println!("Room {room_id} is active.");
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
    println!("Room {room_id} deleted.");
}