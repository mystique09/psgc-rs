use rbatis::{DefaultPool, RBatis};
use rbdc_pg::{driver::PgDriver, options::PgConnectOptions};
use std::str::FromStr;
use std::sync::Arc;
use tracing::info;

use crate::config::db_config::DatabaseConfig;

pub async fn create_db_pool(config: &DatabaseConfig) -> anyhow::Result<Arc<RBatis>> {
    let db = RBatis::new();
    let db = Arc::new(db);

    let driver = rbdc_pg::driver::PgDriver {};

    let conn_options = PgConnectOptions::from_str(&config.db_url)?;

    db.init_option::<PgDriver, PgConnectOptions, DefaultPool>(driver, conn_options)?;

    info!("Pre-warming database connection pool...");
    let _ = db.acquire().await?;
    info!("Database connection pool warmed up successfully");

    Ok(db)
}
