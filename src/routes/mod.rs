mod health_check;
mod subscriptions;

use axum::AddExtensionLayer;
pub use health_check::*;
use hyper::header::HeaderName;
use hyper::{Body, Request};
use sqlx::PgPool;
pub use subscriptions::*;

use axum::routing::post;
use axum::{self, routing::get, Router};
use tower_http::request_id::{
    MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use tower_http::trace::TraceLayer;
use uuid::Uuid;

#[derive(Clone)]
pub struct ApiContext {
    db: PgPool,
}

pub fn app(db: PgPool) -> Router {
    let x_request_id = HeaderName::from_static("x-request-id");
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(AddExtensionLayer::new(ApiContext { db }))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .and_then(|id| id.header_value().to_str().ok())
                    .unwrap_or_default();

                tracing::info_span!(
                    "HTTP",
                    http.method = %request.method(),
                    http.url = %request.uri(),
                    request_id = %request_id,
                )
            }),
        )
        .layer(SetRequestIdLayer::new(
            x_request_id.clone(),
            MakeRequestUuid,
        ))
        .layer(PropagateRequestIdLayer::new(x_request_id))
}

#[derive(Clone, Copy)]
struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string().parse().unwrap();
        Some(RequestId::new(request_id))
    }
}
