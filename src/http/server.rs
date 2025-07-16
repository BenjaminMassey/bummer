#[tokio::main]
pub async fn start() {
    let settings = crate::settings::get_settings();
    let host = format!("{}:{}", settings.http.address, settings.http.port);
    let listener = tokio::net::TcpListener::bind(&host).await.unwrap();
    println!("HTTP server is running on {host}");
    axum::serve(listener, router()).await.unwrap();
}

pub struct AxumState {
    pub settings: crate::settings::Settings,
}

fn router() -> axum::Router {
    let settings = crate::settings::get_settings();
    let shared_state = std::sync::Arc::new(AxumState{settings});
    axum::Router::new()
        .route("/ping", axum::routing::get(crate::http::routes::ping))
        .route("/checkRoom", axum::routing::get(crate::http::routes::check_room))
        .route("/createRoom", axum::routing::post(crate::http::routes::create_room))
        .with_state(shared_state)
}