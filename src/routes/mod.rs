mod health_check;
mod subscriptions;

use axum::AddExtensionLayer;
pub use health_check::*;
use hyper::{Body, Request};
use sqlx::PgPool;
pub use subscriptions::*;

use axum::routing::post;
use axum::{self, routing::get, Router};
use tower_http::request_id::{MakeRequestId, RequestId};
use tower_http::trace::TraceLayer;
use uuid::Uuid;

#[derive(Clone)]
pub struct ApiContext {
    db: PgPool,
}

pub fn app(db: PgPool) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(AddExtensionLayer::new(ApiContext { db }))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                tracing::debug_span!(
                    "HTTP",
                    http.method = %request.method(),
                    http.url = %request.uri(),
                    request_id = %Uuid::new_v4(),
                )
            }),
        )
}

#[derive(Clone, Copy)]
struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string().parse().unwrap();
        Some(RequestId::new(request_id))
    }
}
