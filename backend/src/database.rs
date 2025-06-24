use std::{cmp::max, time::Duration};

use super::config;
use anyhow::Ok;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, Statement};

pub async fn inti() -> anyhow::Result<DatabaseConnection> {
    let config = config::get().database();

    let mut opt = ConnectOptions::new(format!(
        "postgres://{}:{}@{}:{}/{}",
        config.user(),
        config.password(),
        config.host(),
        config.port(),
        config.database(),
    ));

    let cpus = num_cpus::get() as u32;
    opt.min_connections(max(cpus * 4, 10))
        .max_connections(max(cpus * 8, 20))
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(3600))
        .sqlx_logging(false)
        .set_schema_search_path(config.schema());

    let db = Database::connect(opt).await?;
    db.ping().await?;
    tracing::info!("Database connection established");
    log_database_version(&db).await?;

    Ok(db)
}

async fn log_database_version(db: &DatabaseConnection) -> anyhow::Result<()> {
    let version = db
        .query_one(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT version();",
        ))
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to retrieve database version"))?;

    let v = version.try_get_by_index::<String>(0)?;
    tracing::info!("Database version: {v}");
    Ok(())
}
