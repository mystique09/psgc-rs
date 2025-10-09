use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

use crate::config::db_config::DatabaseConfig;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./src/database/migrations/");
}

pub async fn migrator(config: &DatabaseConfig) -> anyhow::Result<()> {
    let conn_url = &config.db_url;
    let manager = PostgresConnectionManager::new_from_stringlike(conn_url, NoTls)?;
    let pool = Pool::builder().max_size(15).build(manager).await?;
    let mut conn = pool.dedicated_connection().await?;

    let report = embedded::migrations::runner().run_async(&mut conn).await?;

    for migrated in report.applied_migrations() {
        println!("Applied migration: {}", migrated);
    }

    Ok(())
}
