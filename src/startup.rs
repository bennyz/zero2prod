use axum::{routing::IntoMakeService, Router, Server};
use hyper::server::conn::AddrIncoming;
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::app;

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, Box<dyn std::error::Error>> {
    let app = app(db_pool);
    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());
    Ok(server)
}
