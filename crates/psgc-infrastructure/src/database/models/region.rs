use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::database::DatabaseSeedError;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, bon::Builder)]
pub struct Region {
    id: uuid::Uuid,
    code: String,
    correspondence_code: String,
    name: String,
    designation: String,
    population: u64,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

rbatis::crud!(Region {});

pub async fn seed_regions(db: &rbatis::RBatis) -> Result<(), DatabaseSeedError> {
    let regions = include_str!("../data/json/regions.json");
    let regions = serde_json::from_str::<Vec<Region>>(regions)
        .map_err(|e| DatabaseSeedError::Serialization(e))?;

    Region::insert_batch(db, &regions, 100)
        .await
        .map_err(|e| DatabaseSeedError::DbError(e))?;

    Ok(())
}
