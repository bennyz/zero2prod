use axum::{routing::IntoMakeService, Router, Server};
use hyper::server::conn::AddrIncoming;
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::app;

pub async fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, std::io::Error> {
    let app = app(db_pool);
    let server = axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service());

    Ok(server)
}
