use axum::http::StatusCode;
use axum_test::{
    TestServer,
    multipart::{MultipartForm, Part},
};

use video_queue::{api, db};

#[tokio::test]
async fn upload_video_should_create_video_and_job() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap();

    let pool = db::create_pool(&database_url).await.unwrap();

    let app = api::create_router(pool.clone());

    let server = TestServer::new(app);

    let form = MultipartForm::new().add_part(
        "video",
        Part::bytes(b"fake video bytes".as_slice()).file_name("test.mp4"),
    );

    let response = server.post("/upload-video").multipart(form).await;

    response.assert_status(StatusCode::CREATED);

    let saved = tokio::fs::read("storage/input/test.mp4").await.unwrap();

    assert_eq!(saved, b"fake video bytes");
    let video_id: i64 = response.text().parse::<i64>().unwrap();
    sqlx::query("SELECT * FROM jobs WHERE video_id = $1")
        .bind(video_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    tokio::fs::remove_file("storage/input/test.mp4").await.ok();

    sqlx::query("DELETE FROM jobs WHERE video_id = $1")
        .bind(video_id)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM videos WHERE id = $1")
        .bind(video_id)
        .execute(&pool)
        .await
        .unwrap();
}
