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
