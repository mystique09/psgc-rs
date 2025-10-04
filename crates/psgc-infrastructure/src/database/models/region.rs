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

#[derive(Debug, Serialize, Deserialize, bon::Builder)]
struct RegionData {
    code: String,
    correspondence_code: String,
    name: String,
    designation: String,
    population: u64,
}

pub async fn seed_regions(db: &rbatis::RBatis) -> Result<(), DatabaseSeedError> {
    let regions = include_str!("../data/json/regions.json");

    let regions = serde_json::from_str::<Vec<RegionData>>(regions)
        .map_err(|e| DatabaseSeedError::Serialization(e))?;

    let regions = regions
        .iter()
        .map(|r| {
            Region::builder()
                .id(uuid::Uuid::now_v7())
                .name(r.name.to_owned())
                .code(r.code.to_owned())
                .correspondence_code(r.correspondence_code.to_owned())
                .designation(r.designation.to_owned())
                .population(r.population)
                .created_at(chrono::Utc::now())
                .updated_at(chrono::Utc::now())
                .build()
        })
        .collect::<Vec<_>>();

    Region::insert_batch(db, &regions, 100)
        .await
        .map_err(|e| DatabaseSeedError::DbError(e))?;

    Ok(())
}
