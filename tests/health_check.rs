use std::net::TcpListener;

use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use tower::ServiceExt;
use uuid::Uuid;
use zero2prod::{
    configuration::{get_configuration, DatabaseSettings},
    routes::app,
    startup::run,
};

pub struct TestApp {
    pub pool: PgPool,
    pub address: String,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone())
        .await
        .expect("Failed to run server");

    let _ = tokio::spawn(server);

    TestApp {
        pool: connection_pool,
        address,
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}

#[tokio::test]
async fn health_check_test() {
    let app = spawn_app().await;
    let client = hyper::client::Client::new();
    let req = Request::builder()
        .method(Method::GET)
        .uri(format!("{}/health_check", app.address))
        .body(Body::empty())
        .unwrap();

    let response = client.request(req).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = hyper::client::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let req = Request::builder()
        .method(Method::POST)
        .uri(format!("{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body.into())
        .unwrap();
    let response = client.request(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = hyper::client::Client::new();

    let cases = vec![
        ("name=le%20guin", "missing field `email`"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in cases {
        let req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}/subscriptions", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body.into())
            .unwrap();
        let response = client.request(req).await.unwrap();
        let status = response.status();
        let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        println!("body {}", body);
        // TODO check error message

        assert_eq!(status, StatusCode::BAD_REQUEST);
    }
}
