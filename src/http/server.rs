#[tokio::main]
pub async fn start(mpsc_sender: std::sync::mpsc::Sender<String>) {
    let settings = crate::settings::get_settings();
    let host = format!("{}:{}", settings.http.address, settings.http.port);
    let listener = tokio::net::TcpListener::bind(&host).await.unwrap();
    println!("HTTP server is running on {host}");
    axum::serve(listener, router(settings, mpsc_sender))
        .await
        .unwrap();
}

pub struct AxumState {
    pub _settings: crate::settings::Settings,
    pub mpsc_sender: std::sync::mpsc::Sender<String>,
}

fn router(
    settings: crate::settings::Settings,
    mpsc_sender: std::sync::mpsc::Sender<String>,
) -> axum::Router {
    let shared_state = std::sync::Arc::new(AxumState {
        _settings: settings,
        mpsc_sender,
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
