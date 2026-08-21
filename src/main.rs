mod handlers;

use axum::{
    routing::{get},
    Router,
};

#[tokio::main]
async fn main() {
    // build the router: when a GET arrives on /health, calls handlers::health::health
    let app = Router::new()
        .route("/health", get(handlers::health::health));

    // open port 8080 on every interface
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
