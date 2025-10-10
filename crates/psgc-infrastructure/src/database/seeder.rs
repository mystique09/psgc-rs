use crate::{
    config::db_config::DatabaseConfig,
    database::models::{
        barangay::seed_barangays, city::seed_cities, municipality::seed_municipalities,
        province::seed_provinces, region::seed_regions,
    },
};

pub async fn seeder(config: &DatabaseConfig) -> anyhow::Result<()> {
    let db = rbatis::RBatis::new();

    db.init(rbdc_pg::driver::PgDriver {}, &config.db_url)
        .unwrap();

    println!("Seeding database...");
    seed_regions(&db).await?;
    seed_provinces(&db).await?;
    seed_cities(&db).await?;
    seed_municipalities(&db).await?;
    seed_barangays(&db).await?;

    Ok(())
}
