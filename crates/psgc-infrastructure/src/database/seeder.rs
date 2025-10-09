use crate::{config::db_config::DatabaseConfig, database::models::region::seed_regions};

pub async fn seeder(config: &DatabaseConfig) -> anyhow::Result<()> {
    let db = rbatis::RBatis::new();

    db.init(rbdc_pg::driver::PgDriver {}, &config.db_url)
        .unwrap();

    println!("Seeding database...");
    seed_regions(&db).await?;

    Ok(())
}
