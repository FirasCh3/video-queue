use tokio::{io, process::Command};

use std::error::Error;

pub async fn process_job(
    input_path: &String,
    original_filename: &String,
) -> Result<(), Box<dyn Error>> {
    let output = Command::new("ffmpeg")
        .args(&[
            "-i",
            input_path.as_str(),
            "-vf",
            "scale=1080:1920",
            ("storage/output/".to_string() + original_filename.as_str()).as_str(),
            "-y",
            "-s",
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
