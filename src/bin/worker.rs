use std::error::Error;

use sqlx::postgres::PgListener;
use video_queue::{db, worker::run_worker};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tokio::fs::create_dir_all("storage/output").await.unwrap();
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = db::create_pool(&database_url).await.unwrap();
    let listener = PgListener::connect(&database_url).await?;
    run_worker(&pool, listener).await?;
    Ok(())
}
