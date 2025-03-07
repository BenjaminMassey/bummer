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
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error 1.".to_owned());
    } else {
        let auth_key = auth.unwrap();
        if &payload.auth_key != &auth_key {
            return (axum::http::StatusCode::UNAUTHORIZED, "Authorization Failure.".to_owned());
        }
    }

    // Verify that UDP call to create room coming from app itself
    let secret = std::fs::read_to_string("secret.key");
    if let Err(e) = secret {
        println!("Secret key read error: {e}");
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error 2.".to_owned());
    } else {
        let room_id = payload.room_id.clone();
        let secret_key = secret.unwrap();
        let response: String = crate::udp::client::create_room(&room_id, &secret_key);
        let code = {
            if &response == "success" {
                axum::http::StatusCode::OK
            } else {
                axum::http::StatusCode::BAD_REQUEST
            }
        };
        if code == axum::http::StatusCode::OK {
            let _handler = std::thread::spawn(move || {
                handle_room_state(&room_id, &secret_key);
            });
        }
        return (code, response);
    }
}

pub async fn check_room(
    axum::extract::Json(payload): axum::extract::Json<crate::http::data::CheckRoom>,
) -> impl axum::response::IntoResponse {
    // Check that from a valid client
    let auth = std::fs::read_to_string("auth.key");
    if let Err(e) = auth {
        println!("Auth key read error: {e}");
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error 3.".to_owned());
    } else {
        let auth_key = auth.unwrap();
        if &payload.auth_key != &auth_key {
            return (axum::http::StatusCode::UNAUTHORIZED, "Authorization Failure.".to_owned());
        }
    }

    // Verify that UDP call to create room coming from app itself
    let secret = std::fs::read_to_string("secret.key");
    if let Err(e) = secret {
        println!("Secret key read error: {e}");
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error 4.".to_owned());
    } else {
        let secret_key = secret.unwrap();
        let response: String = crate::udp::client::check_room(&payload.room_id, &secret_key);
        let code = {
            if &response == "success" {
                axum::http::StatusCode::OK
            } else {
                axum::http::StatusCode::BAD_REQUEST
            }
        };
        return (code, response);
    }
}

fn handle_room_state(room_id: &str, secret_key: &str) {
    //println!("Starting handler for room {room_id}.");
    loop {
        let response: String = crate::udp::client::delete_room(room_id, secret_key);
        if &response == "success" {
            break;
        }
        let _ = crate::udp::client::delete_players(room_id, secret_key);
        //println!("Room {room_id} is active.");
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
    println!("Room {room_id} deleted.");
}