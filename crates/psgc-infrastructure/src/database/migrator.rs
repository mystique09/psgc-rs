use std::sync::Arc;

use flyway::{MigrationRunner, MigrationStore};
use flyway_rbatis::RbatisMigrationDriver;
use rbatis::RBatis;

pub async fn migrator<S: MigrationStore>(store: S, rbatis: Arc<RBatis>) -> anyhow::Result<()> {
    let migration_driver = Arc::new(RbatisMigrationDriver::new(rbatis, None));
    let migration_runner = MigrationRunner::new(
        store,
        migration_driver.clone(),
        migration_driver.clone(),
        false,
    );
    migration_runner.migrate().await?;

    Ok(())
}
