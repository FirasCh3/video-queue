use sqlx::PgPool;

pub async fn create_job(pool: &PgPool, video_id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO jobs (video_id) VALUES ($1)")
        .bind(video_id)
        .execute(pool)
        .await?;
    Ok(())
}
