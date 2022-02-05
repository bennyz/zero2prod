use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use sqlx::{postgres::PgPoolOptions, Executor, PgPool};
use tower::ServiceExt;
use uuid::Uuid;
use zero2prod::{configuration::get_configuration, routes::app};

async fn get_db() -> PgPool {
    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_string = configuration.database.connection_string_without_db();
    let pool = PgPoolOptions::new()
        .connect(&connection_string)
        .await
        .unwrap();
    pool.execute(
        format!(
            r#"CREATE DATABASE "{}";"#,
            configuration.database.database_name
        )
        .as_str(),
    )
    .await
    .unwrap();

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    println!("{:?}", configuration.database.database_name);
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

#[tokio::test]
async fn health_check_test() {
    let app = app(get_db().await);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health_check")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let db = get_db().await;
    let app = app(db.clone());

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/subscribtions")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(body.into())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&db)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let app = app(get_db().await);

    let cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in cases {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/subscribtions")
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(invalid_body.into())
                    .unwrap(),
            )
            .await
            .unwrap();

        let status = response.status();
        let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }
}
