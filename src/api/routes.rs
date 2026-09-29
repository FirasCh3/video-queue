use axum::{
    routing::get,
    Router,
};

use super::handlers::hello;

pub fn create_router() -> Router {
    Router::new().route("/", get(hello))
}