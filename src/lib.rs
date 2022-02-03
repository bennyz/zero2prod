use axum::{self, routing::get, Router};
use axum::{http::StatusCode, response::IntoResponse};
use std::error;
use std::net::SocketAddr;

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

pub fn app() -> Router {
    Router::new().route("/health_check", get(health_check))
}

pub async fn run() -> Result<(), Box<dyn error::Error>> {
    let app = app();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
