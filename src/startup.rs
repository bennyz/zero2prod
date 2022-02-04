use std::{error, net::SocketAddr};

use crate::routes::app;

pub async fn run() -> Result<(), Box<dyn error::Error>> {
    let app = app();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
