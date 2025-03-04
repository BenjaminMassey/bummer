#[tokio::main]
pub async fn start() {
    let host = format!("{}:{}", crate::ADDRESS, crate::HTTP_PORT);
    let listener = tokio::net::TcpListener::bind(&host).await.unwrap();
    println!("HTTP server is running on {host}");
    axum::serve(listener, router()).await.unwrap();
}

fn router() -> axum::Router {
    axum::Router::new()
        .route("/ping", axum::routing::get(crate::http::routes::ping))
        .route("/createRoom", axum::routing::post(crate::http::routes::create_room))
}