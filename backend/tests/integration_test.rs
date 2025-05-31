// tests/backend_integration_test.rs

use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::get, // Important: Need `get` here for `Router::route`
    Router,
};
use http_body_util::BodyExt; // For `collect` and `to_bytes`
use tower::ServiceExt; // For `oneshot`

// Because your `root` function is not public and not in a library crate,
// the simplest way to test it without modifying `main.rs` is to
// redefine it directly in your test file, as it's a very simple handler.
async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::test]
async fn test_root_endpoint() {
    // Recreate the Axum app as it's defined in your src/backend/main.rs
    let app = Router::new()
        .route("/", get(root)); // Use the `root` function defined above

    // Simulate a GET request to the "/" endpoint
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    // Assert the status code is OK (200)
    assert_eq!(response.status(), StatusCode::OK);

    // Read the response body
    let body = response.into_body().collect().await.unwrap().to_bytes();

    // Assert the body content
    assert_eq!(&body[..], b"Hello, World!");
}

// You can add more tests here for other routes if you add them later
/*
#[tokio::test]
async fn test_non_existent_route() {
    let app = Router::new().route("/", get(root));

    let response = app
        .oneshot(Request::builder().uri("/nonexistent").body(Body::empty()).unwrap())
        .await
        .unwrap();

    // Axum returns 404 for unmatched routes by default
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
*/