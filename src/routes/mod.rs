mod health_check;
mod subscriptions;

use axum::AddExtensionLayer;
pub use health_check::*;
use sqlx::PgPool;
pub use subscriptions::*;

use axum::routing::post;
use axum::{self, routing::get, Router};

#[derive(Clone)]
pub struct ApiContext {
    db: PgPool,
}

pub fn app(db: PgPool) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscribtions", post(subscribe))
        .layer(AddExtensionLayer::new(ApiContext { db }))
}
