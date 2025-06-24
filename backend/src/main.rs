use axum::{Router, routing};

mod app;
mod config;
mod database;
mod logger;
mod server;
mod api;
mod error;
mod response;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new().route("/vehicle", routing::get(vehicle_get));
    app::run(router).await
}

async fn vehicle_get() -> &'static str {
    "Vehicle endpoint"
}

