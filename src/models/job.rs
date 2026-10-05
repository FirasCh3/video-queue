#[derive(sqlx::FromRow)]
pub struct Job {
    pub id: i64,
    pub video_id: i64,
    pub status: String,
    pub attempts: i64,
    pub lease_until: Option<chrono::NaiveDateTime>,
    pub started_at: Option<chrono::NaiveDateTime>,
    pub completed_at: Option<chrono::NaiveDateTime>,
}
