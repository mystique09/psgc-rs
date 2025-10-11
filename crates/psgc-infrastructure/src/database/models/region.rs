use crate::database::{
    DatabaseSeedError,
    generators::{DateTimeUtcExt, UuidExt, datetime_utc_now, uuid_now},
};
use serde::{Deserialize, Serialize};
use tracing::info;

#[allow(dead_code)]
#[derive(Debug, Default, Serialize, Deserialize, bon::Builder)]
pub struct Region {
    pub id: rbatis::rbdc::Uuid,
    pub code: String,
    pub correspondence_code: String,
    pub name: String,
    pub designation: String,
    pub population: u64,
    pub created_at: rbatis::rbdc::DateTime,
    pub updated_at: rbatis::rbdc::DateTime,
}

rbatis::crud!(Region {}, "regions");
rbatis::impl_select!(Region {select_by_codename(codename: &str) -> Option => "`where code = #{codename} limit 1`" }, "regions");
rbatis::impl_select_page!(Region {list_all() => ""}, "regions");

#[derive(Debug, Serialize, Deserialize, bon::Builder)]
struct RegionData {
    name: String,
    designation: String,
    code: String,
    correspondence_code: String,
    population: u64,
}

pub async fn seed_regions(db: &rbatis::RBatis) -> Result<(), DatabaseSeedError> {
    info!("Sedding regions...");
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
                .created_at(datetime_utc_now())
                .updated_at(datetime_utc_now())
                .build()
        })
        .collect::<Vec<_>>();

    Region::insert_batch(db, &regions, 100)
        .await
        .map_err(|e| DatabaseSeedError::DbError(e))?;

    info!("Added {} regions to database", regions.len());

    Ok(())
}

impl From<Region> for psgc_domain::models::region::Region {
    fn from(value: Region) -> Self {
        Self::builder()
            .id(value.id.inner())
            .name(value.name)
            .code(value.code)
            .correspondence_code(value.correspondence_code)
            .population(value.population)
            .designation(value.designation)
            .created_at(value.created_at.inner())
            .updated_at(value.updated_at.inner())
            .build()
    }
}
