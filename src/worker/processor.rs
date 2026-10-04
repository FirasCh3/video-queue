use tokio::{io, process::Command};

use crate::models::job_notification::JobNotification;
use std::error::Error;

pub async fn process_job(job_notif: JobNotification) -> Result<(), Box<dyn Error>> {
    let output = Command::new("ffmpeg")
        .args(&[
            "-i".to_string(),
            job_notif.input_path,
            "-vf".to_string(),
            "scale=1080:1920".to_string(),
            "storage/output/".to_string() + job_notif.original_filename.as_str(),
        ])
        .output()
        .await?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err((io::Error::other(format!("FFmpeg failed: {}", stderr))).into());
    }
    println!("{:?}", output);
    Ok(())
}
