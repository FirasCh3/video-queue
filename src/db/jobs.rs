use chrono::{Duration, NaiveDateTime, Utc};
use sqlx::PgPool;

use crate::models::job::Job;
pub async fn create_job(pool: &PgPool, video_id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO jobs (video_id) VALUES ($1)")
        .bind(video_id)
        .execute(pool)
        .await?;
    Ok(())
}
/*pub async fn fetch_pending_job(pool: &PgPool) -> Result<vec![Job], sqlx::Error> {
    let job: vec![Job] = sqlx::query_as("SELECT * FROM jobs WHERE status = $1")
        .bind("pending")
        .fetch_all(pool)
        .await?;
    Ok(job)
}*/
pub async fn claim_job(pool: &PgPool) -> Result<Option<Job>, sqlx::Error> {
    let current_time = Utc::now().naive_utc();
    let mut tx = pool.begin().await?;
    let job: Option<Job> = sqlx::query_as(
        "SELECT * FROM jobs WHERE status = 'pending' LIMIT 1 FOR UPDATE SKIP LOCKED",
    )
    .fetch_optional(&mut *tx)
    .await?;
    match job {
        Some(job) => {
            sqlx::query(
            "UPDATE jobs set status = 'in_progress', started_at = $2, lease_until = $3 WHERE id = $1",
        )
        .bind(job.id)
        .bind(current_time + Duration::minutes(5))
        .execute(&mut *tx)
        .await?;
            tx.commit().await?;
            Ok(Some(job))
        }
        None => Ok(None),
    }
}
