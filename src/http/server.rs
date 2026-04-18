#[tokio::main]
pub async fn start(
    receive_from_udp: std::sync::mpsc::Receiver<String>,
    send_to_udp: std::sync::mpsc::Sender<String>,
) {
    let settings = crate::settings::get_settings();
    let host = format!("{}:{}", settings.http.address, settings.http.port);
    let listener = tokio::net::TcpListener::bind(&host).await.unwrap();
    println!("HTTP server is running on {host}");
    axum::serve(listener, router(settings, receive_from_udp, send_to_udp))
        .await
        .unwrap();
}

pub struct AxumState {
    pub _settings: crate::settings::Settings,
    pub receive_from_udp: std::sync::Mutex<std::sync::mpsc::Receiver<String>>,
    pub send_to_udp: std::sync::mpsc::Sender<String>,
}

fn router(
    settings: crate::settings::Settings,
    receive_from_udp: std::sync::mpsc::Receiver<String>,
    send_to_udp: std::sync::mpsc::Sender<String>,
) -> axum::Router {
    let shared_state = std::sync::Arc::new(AxumState {
        _settings: settings,
        receive_from_udp: std::sync::Mutex::new(receive_from_udp),
        send_to_udp,
    });
    axum::Router::new()
        .route("/ping", axum::routing::get(crate::http::routes::ping))
        .route(
            "/checkRoom",
            axum::routing::get(crate::http::routes::check_room),
        )
        .route(
            "/createRoom",
            axum::routing::post(crate::http::routes::create_room),
        )
        .with_state(shared_state)
}
