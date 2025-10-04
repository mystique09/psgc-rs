use std::sync::Arc;

use rbatis::RBatis;

use crate::config::db_config::DatabaseConfig;

pub fn create_db_pool(config: &DatabaseConfig) -> anyhow::Result<Arc<RBatis>> {
    let db = rbatis::RBatis::new();
    let db = Arc::new(db);

    db.init(rbdc_pg::driver::PgDriver {}, &config.db_url)?;

    Ok(db)
}
