use axum::{extract::Form, response::IntoResponse};
use hyper::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(input: Form<FormData>) -> impl IntoResponse {
    StatusCode::OK
}
