use axum::{
    extract::{Extension, Form},
    response::IntoResponse,
};
use hyper::StatusCode;
use serde::Deserialize;
use sqlx::types::chrono::Utc;
use tracing::Instrument;
use uuid::Uuid;

use super::ApiContext;

#[derive(Deserialize, Debug)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(input: Form<FormData>, ctx: Extension<ApiContext>) -> impl IntoResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!("Adding a new subscriber.", %request_id, subscriber_email = %input.email, subscriber_name= %input.name);
    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!("Saving new subscriber details in the database");

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
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscriber details have been saved",
                request_id
            );
            StatusCode::OK
        }
        Err(e) => {
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
