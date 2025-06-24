use axum::Router;
use sea_orm::DatabaseConnection;

use crate::{config, database, logger, server};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

pub async fn run(router: Router<AppState>) -> anyhow::Result<()> {
    logger::init();
    tracing::info!("Starting application...");
    let db = database::inti().await?;
    let state = AppState::new(db);
    let server = server::Server::new(config::get().server());
    server.start(state, router).await
}
