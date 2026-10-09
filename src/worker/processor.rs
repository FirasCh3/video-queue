use tokio::{io, process::Command};

use std::error::Error;

pub async fn process_job(
    input_path: String,
    original_filename: String,
) -> Result<(), Box<dyn Error>> {
    let output = Command::new("ffmpeg")
        .args(&[
            "-i".to_string(),
            input_path,
            "-vf".to_string(),
            "scale=1080:1920".to_string(),
            "storage/output/".to_string() + original_filename.as_str(),
            "-y".to_string(),
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
