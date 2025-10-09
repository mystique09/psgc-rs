use rbatis::RBatis;
use std::sync::Arc;

use crate::config::db_config::DatabaseConfig;

pub fn create_db_pool(config: &DatabaseConfig) -> anyhow::Result<Arc<RBatis>> {
    let db = rbatis::RBatis::new();
    let db = Arc::new(db);

    let driver = rbdc_pg::driver::PgDriver {};
    let conn_url = &config.db_url;

    db.init(driver, &conn_url)?;

    Ok(db)
}
