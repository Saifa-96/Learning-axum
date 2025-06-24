use axum::{routing, Router};

use crate::app::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", routing::get(handler))
}

async fn handler() -> &'static str {
    "Hello, World!"
}