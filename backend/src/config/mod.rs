use std::sync::LazyLock;
use anyhow::Context;
use config::Config;
use serde::Deserialize;

pub mod server;
pub mod database;

static CONFIG: LazyLock<AppConfig> = LazyLock::new(|| AppConfig::load().expect("Failed to load configuration"));

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    server: server::ServerConfig,
    database: database::DatabaseConfig,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        Config::builder()
            .add_source(
                config::File::with_name("application")
                    .format(config::FileFormat::Yaml)
                    .required(true),
            )
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            )
            .build()
            .with_context(|| anyhow::anyhow!("Failed to build configuration"))?
            .try_deserialize()
            .with_context(|| anyhow::anyhow!("Failed to deserialize configuration"))
    }

    pub fn server(&self) -> &server::ServerConfig {
        &self.server
    }
    pub fn database(&self) -> &database::DatabaseConfig {
        &self.database
    }
}

pub fn get() -> &'static AppConfig {
    &CONFIG
}
