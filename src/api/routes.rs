use axum::{Router, extract::DefaultBodyLimit, routing::post};
use sqlx::PgPool;

use super::handlers::create_video;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/upload-video", post(create_video))
        .layer(DefaultBodyLimit::max(1000 * 1024 * 1024))
        .with_state(pool)
}
