use chrono::Local;
use sqlx::PgPool;
pub async fn insert_video(
    pool: &PgPool,
    filename: &str,
    input_path: &str,
) -> Result<i64, sqlx::Error> {
    let created_at = Local::now().naive_local();
    let id = sqlx::query_scalar(
        "INSERT INTO videos(original_filename, input_path, created_at) VALUES($1, $2, $3) RETURNING id",
    )
    .bind(filename)
    .bind(input_path)
    .bind(created_at)
    .fetch_one(pool)
    .await?;
    Ok(id)
}

pub async fn fetch_video_input_path(
    job_id: i64,
    pool: &PgPool,
) -> Result<(String, String), sqlx::Error> {
    let (input_path, original_filename) = sqlx::query_as(
        "SELECT input_path, original_filename FROM videos vid INNER JOIN jobs j on j.video_id = vid.id WHERE j.id = $1",
    )
    .bind(job_id)
    .fetch_one(pool).await?;
    Ok((input_path, original_filename))
}
