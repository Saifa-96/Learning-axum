use std::net::SocketAddr;

use anyhow::Ok;
use axum::Router;
use tokio::net::TcpListener;

use super::config::server::ServerConfig;
use crate::app::AppState;

pub struct Server {
    config: &'static ServerConfig,
}

impl Server {
    pub fn new(config: &'static ServerConfig) -> Self {
        Self { config }
    }

    pub async fn start(&self, state: AppState, router: Router<AppState>) -> anyhow::Result<()> {
        let port = self.config.port();
        let router = self.build_router(state, router);
        let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
        tracing::info!("Listening on {}", listener.local_addr()?);
        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;
        Ok(())
    }

    fn build_router(&self, state: AppState, router: Router<AppState>) -> Router {
        Router::new().merge(router).with_state(state)
    }
}
