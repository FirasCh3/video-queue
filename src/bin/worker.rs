use sqlx::postgres::PgListener;
use video_queue::{models::job_notification::JobNotification, worker::process_job};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tokio::fs::create_dir_all("storage/output").await.unwrap();
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let mut listener = PgListener::connect(&database_url).await?;
    listener.listen("job_notif").await?;
    loop {
        let notification = listener.recv().await?;
        let job: JobNotification = match serde_json::from_str(notification.payload()) {
            Ok(job) => job,
            Err(error) => {
                eprintln!("notification error: {}", error);
                continue;
            }
        };
        match process_job(job).await {
            Ok(()) => {}
            Err(error) => {
                eprintln!("processing job error: {}", error);
            }
        }
    }
}
