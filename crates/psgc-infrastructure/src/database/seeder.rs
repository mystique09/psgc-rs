use rbatis::RBatis;
use std::sync::Arc;

use crate::database::models::{
    barangay::seed_barangays, city::seed_cities, municipality::seed_municipalities,
    province::seed_provinces, region::seed_regions,
};

pub async fn seeder(db: Arc<RBatis>) -> anyhow::Result<()> {
    seed_regions(&db).await?;
    seed_provinces(&db).await?;
    seed_cities(&db).await?;
    seed_municipalities(&db).await?;
    seed_barangays(&db).await?;

    Ok(())
}
