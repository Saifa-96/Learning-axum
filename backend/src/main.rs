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
    let router = api::create_router();
    app::run(router).await
}
