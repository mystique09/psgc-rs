use crate::{
    config::db_config::DatabaseConfig,
    database::{DatabaseSeedError, models::region::seed_regions},
};

pub async fn seeder(config: &DatabaseConfig) -> Result<(), DatabaseSeedError> {
    let db = rbatis::RBatis::new();

    db.init(rbdc_pg::driver::PgDriver {}, &config.db_url)
        .unwrap();

    seed_regions(&db).await?;

    Ok(())
}
