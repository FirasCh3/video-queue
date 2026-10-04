use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JobNotification {
    pub id: i64,
    pub input_path: String,
    pub original_filename: String,
}
