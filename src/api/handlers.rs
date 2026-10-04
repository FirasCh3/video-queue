use axum::{
    extract::{Multipart, State},
    http::StatusCode,
};
use sqlx::PgPool;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::db;

pub async fn create_video(
    State(pool): State<PgPool>,
    mut multipart: Multipart,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    fs::create_dir_all("storage/input")
        .await
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;
    while let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|error| (StatusCode::BAD_REQUEST, error.to_string()))?
    {
        let filename = field.file_name().unwrap_or("video.mp4").to_string();
        let input_path = format!("storage/input/{}", filename);
        let mut file = File::create(&input_path)
            .await
            .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;
        while let Some(chunk) = field
            .chunk()
            .await
            .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?
        {
            file.write_all(&chunk)
                .await
                .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?
        }
        let video_id = db::insert_video(&pool, &filename, &input_path)
            .await
            .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;
        db::create_job(&pool, video_id)
            .await
            .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;
        return Ok((StatusCode::CREATED, video_id.to_string()));
    }
    Err((StatusCode::BAD_REQUEST, "No video was uploaded".to_string()))
}
