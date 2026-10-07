use std::error::Error;

use sqlx::{PgPool, postgres::PgListener};

use crate::{
    db::{claim_job, fetch_video_input_path},
    worker::process_job,
};

pub async fn run_worker(pool: &PgPool, mut listener: PgListener) -> Result<(), Box<dyn Error>> {
    loop {
        match claim_job(pool).await {
            Ok(Some(job)) => match fetch_video_input_path(job.id, pool).await {
                Ok((input_path, original_filename)) => {
                    match process_job(input_path, original_filename).await {
                        Ok(()) => {}
                        Err(error) => {
                            eprintln!("processing job error: {}", error);
                        }
                    }
                }
                Err(error) => {
                    eprintln!("error fetching input path: {}", error);
                }
            },
            Ok(None) => break,

            Err(error) => {
                eprintln!("processing job error: {}", error);
            }
        }
    }
    listener.listen("job_notif").await?;
    loop {
        let notification = listener.recv().await?;
        match serde_json::from_str(notification.payload()) {
            Ok(job_id) => match fetch_video_input_path(job_id, pool).await {
                Ok((input_path, original_filename)) => {
                    match process_job(input_path, original_filename).await {
                        Ok(()) => {}
                        Err(error) => {
                            eprintln!("processing job error: {}", error);
                        }
                    }
                }
                Err(error) => {
                    eprintln!("error fetching input path: {}", error);
                }
            },
            Err(error) => {
                eprintln!("notification error: {}", error);
                continue;
            }
        };
    }
}
