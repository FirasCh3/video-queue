use axum::{
    body::Body,
    http::{Request, StatusCode, header::CONTENT_TYPE},
};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

use video_queue::api;

#[tokio::test]
async fn upload_video_should_save_file() {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let app = api::create_router(pool.clone());

    let boundary = "TEST_BOUNDARY";

    let mut body = format!(
        "--{boundary}\r\n\
         Content-Disposition: form-data; name=\"video\"; filename=\"test.mp4\"\r\n\
         Content-Type: video/mp4\r\n\
         \r\n"
    )
    .into_bytes();

    body.extend_from_slice(b"fake video bytes");

    body.extend_from_slice(
        format!("\r\n--{boundary}--\r\n").as_bytes()
    );

    let request = Request::builder()
        .method("POST")
        .uri("/upload-video")
        .header(
            CONTENT_TYPE,
            format!("multipart/form-data; boundary={boundary}"),
        )
        .body(Body::from(body))
        .unwrap();

    let response = app
        .oneshot(request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let saved = tokio::fs::read("storage/input/test.mp4")
        .await
        .unwrap();

    assert_eq!(saved, b"fake video bytes");


    tokio::fs::remove_file("storage/input/test.mp4")
        .await
        .ok();

    sqlx::query(
        "DELETE FROM videos WHERE original_filename = $1"
    )
    .bind("test.mp4")
    .execute(&pool)
    .await
    .unwrap();
}