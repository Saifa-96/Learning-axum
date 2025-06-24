use axum::Router;

use crate::app::AppState;

mod user;

pub fn create_router() -> Router<AppState> {
    Router::new().nest(
        "/api",
        Router::new()
            .nest("/user", user::create_router())
            .fallback(async || {
                tracing::warn!("Not found");
            }),
    )
}
