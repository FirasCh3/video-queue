use chrono::{Duration, Utc};
use sqlx::PgPool;

use crate::models::job::Job;
pub async fn create_job(pool: &PgPool, video_id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO jobs (video_id) VALUES ($1)")
        .bind(video_id)
        .execute(pool)
        .await?;
    Ok(())
}
pub async fn complete_job(job_id: i64, pool: &PgPool) -> Result<(), sqlx::Error> {
    let current_time = Utc::now().naive_utc();
    sqlx::query("UPDATE jobs set STATUS = 'complete', completed_at = $2 where id = $1")
        .bind(job_id)
        .bind(current_time)
        .execute(pool)
        .await?;
    Ok(())
}
pub async fn claim_job(pool: &PgPool) -> Result<Option<Job>, sqlx::Error> {
    let current_time = Utc::now().naive_utc();
    let mut tx = pool.begin().await?;
    let job: Option<Job> = sqlx::query_as(
        "SELECT * FROM jobs WHERE status = 'pending' OR (status = 'in_progress' AND lease_until < $1) LIMIT 1 FOR UPDATE SKIP LOCKED",
    )
    .bind(current_time)
    .fetch_optional(&mut *tx)
    .await?;
    match job {
        Some(job) => {
            sqlx::query(
            "UPDATE jobs set status = 'in_progress', started_at = $2, lease_until = $3, attempts = attempts + 1 WHERE id = $1",
        )
        .bind(job.id)
        .bind(current_time)
        .bind(current_time + Duration::minutes(5))
        .execute(&mut *tx)
        .await?;
            tx.commit().await?;
            Ok(Some(job))
        }
        None => Ok(None),
    }
}
pub async fn retry_or_fail_job(pool: &PgPool, job_id: i64) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
            UPDATE jobs
            SET status =
                CASE WHEN attempts < 3 THEN 'pending'
                ELSE 'failed'
            END
            WHERE id = $1
        ",
    )
    .bind(job_id)
    .execute(pool)
    .await?;
    Ok(())
}
