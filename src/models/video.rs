use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateVideoRequest {
    pub original_filename: String,
    pub input_path: String,
}