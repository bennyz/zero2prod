use std::{error, net::SocketAddr};

use sqlx::postgres::PgPoolOptions;

use crate::{configuration, routes::app};

pub async fn run(address: String) -> Result<(), Box<dyn error::Error>> {
    let db = PgPoolOptions::new()
        .connect(
            &configuration::get_configuration()
                .unwrap()
                .database
                .connection_string(),
        )
        .await?;

    let app = app(db);

    let addr = SocketAddr::from(address.parse::<SocketAddr>()?);
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
