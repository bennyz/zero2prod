use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use tower::ServiceExt;
use zero2prod::routes::app;

#[tokio::test]
async fn health_check_test() {
    let app = app();
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
    let app = app();

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
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let app = app();

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
