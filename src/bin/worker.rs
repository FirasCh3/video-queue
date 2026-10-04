use serde::Serialize;
use sqlx::postgres::PgListener;
use video_queue::models::{
    self,
    job_notification::{self, JobNotification},
};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let mut listener = PgListener::connect(&database_url).await?;
    listener.listen("job_notif").await?;
    loop {
        let notification = listener.recv().await?;
        let job: JobNotification = serde_json::from_str(notification.payload()).unwrap();
    }
}
