use axum::{Router, routing::post};
use sqlx::PgPool;

use super::handlers::create_video;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/upload-video", post(create_video))
        .with_state(pool)
}
