use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use hyper::StatusCode;
use serde::Deserialize;
use sqlx::types::chrono::Utc;
use uuid::Uuid;

use super::ApiContext;

#[derive(Deserialize, Debug)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(input: Form<FormData>, ctx: Extension<ApiContext>) -> impl IntoResponse {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        input.email,
        input.name,
        Utc::now()
    )
    .execute(&ctx.db)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
