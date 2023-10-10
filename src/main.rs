use axum::http;
use axum::routing::{get, Router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Port and address we'll use to serve our API:
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    // let port = 3000;
    let addr = format!("0.0.0.0:{}", port);
    let app = Router::new().route("/", get(health_check));

    // Start an Axum server on port 3000.
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn health_check() -> http::StatusCode {
    http::StatusCode::OK
}