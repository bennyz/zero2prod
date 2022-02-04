mod health_check;
mod subscriptions;

pub use health_check::*;
pub use subscriptions::*;

use axum::routing::post;
use axum::{self, routing::get, Router};

pub fn app() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscribtions", post(subscribe))
}
