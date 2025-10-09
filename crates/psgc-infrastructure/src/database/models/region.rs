use crate::database::{DatabaseSeedError, generators::uuid_now};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Default, Serialize, Deserialize, bon::Builder)]
pub struct Region {
    id: rbatis::rbdc::Uuid,
    code: String,
    correspondence_code: String,
    name: String,
    designation: String,
    population: u64,
    created_at: rbatis::rbdc::DateTime,
    updated_at: rbatis::rbdc::DateTime,
}

rbatis::crud!(Region {}, "regions");

#[derive(Debug, Serialize, Deserialize, bon::Builder)]
struct RegionData {
    name: String,
    designation: String,
    code: String,
    correspondence_code: String,
    population: u64,
}

pub async fn seed_regions(db: &rbatis::RBatis) -> Result<(), DatabaseSeedError> {
    println!("Sedding regions...");
    let regions = include_str!("../data/json/regions.json");

    let regions = serde_json::from_str::<Vec<RegionData>>(regions)
        .map_err(|e| DatabaseSeedError::Serialization(e))?;

    let regions = regions
        .iter()
        .map(|r| {
            Region::builder()
                .id(uuid_now())
                .name(r.name.to_owned())
                .code(r.code.to_owned())
                .correspondence_code(r.correspondence_code.to_owned())
                .designation(r.designation.to_owned())
                .population(r.population)
                .created_at(rbatis::rbdc::DateTime::utc())
                .updated_at(rbatis::rbdc::DateTime::utc())
                .build()
        })
        .collect::<Vec<_>>();

    Region::insert_batch(db, &regions, 100)
        .await
        .map_err(|e| DatabaseSeedError::DbError(e))?;

    println!("Added {} regions to database", regions.len());
    Ok(())
}
