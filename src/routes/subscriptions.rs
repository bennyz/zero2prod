use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use hyper::StatusCode;
use serde::Deserialize;
use sqlx::{types::chrono::Utc, PgPool};
use tracing::Instrument;
use uuid::Uuid;

use super::ApiContext;

#[derive(Deserialize, Debug)]
pub struct FormData {
    name: String,
    email: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(input, ctx),
    fields(
    request_id = %Uuid::new_v4(),
    subscriber_email = %input.email,
    subscriber_name= %input.name
    )
)]
pub async fn subscribe(input: Form<FormData>, ctx: Extension<ApiContext>) -> impl IntoResponse {
    match insert_subscriber(&ctx.db, &input).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(input, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, input: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        input.email,
        input.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
