use std::error::Error;

use sqlx::{PgPool, postgres::PgListener};

use crate::{db::claim_job, worker::process_job};

pub async fn run_worker(pool: &PgPool, mut listener: PgListener) -> Result<(), Box<dyn Error>> {
    loop {
        match claim_job(pool).await {
            Ok(Some(job)) => { /* fetch video info using job_id and pass it to processor */ }
            Ok(None) => break,

            Err(error) => {
                eprintln!("processing job error: {}", error);
            }
        }
    }
    listener.listen("job_notif").await?;
    loop {
        let notification = listener.recv().await?;
        let job_id: i64 = match serde_json::from_str(notification.payload()) {
            Ok(job_id) => job_id,
            Err(error) => {
                eprintln!("notification error: {}", error);
                continue;
            }
        };
        /*match process_job(job_id).await {
            Ok(()) => {}
            Err(error) => {
                eprintln!("processing job error: {}", error);
            }
        }*/
    }
}
