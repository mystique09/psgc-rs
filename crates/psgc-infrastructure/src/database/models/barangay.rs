use crate::database::{
    DatabaseSeedError,
    generators::{datetime_utc_now, uuid_now},
    helpers::{get_city_map, get_municipality_map},
};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Default, Serialize, Deserialize, bon::Builder)]
pub struct Barangay {
    pub id: rbatis::rbdc::Uuid,
    pub code: String,
    pub correspondence_code: String,
    pub name: String,
    pub population: u64,
    pub urban_rural: String,
    pub city_id: Option<rbatis::rbdc::Uuid>,
    pub municipality_id: Option<rbatis::rbdc::Uuid>,
    pub district_id: Option<rbatis::rbdc::Uuid>,
    pub created_at: rbatis::rbdc::DateTime,
    pub updated_at: rbatis::rbdc::DateTime,
}

#[derive(Debug, Serialize, Deserialize, bon::Builder)]
struct BarangayData {
    #[serde(rename = "psgc10DigitCode")]
    correspondence_code: String,
    name: String,
    #[serde(rename = "code")]
    code: String,
    #[serde(rename = "urbanRural")]
    urban_rural: String,
}

rbatis::crud!(Barangay {}, "barangays");

pub async fn seed_barangays(db: &rbatis::RBatis) -> Result<(), DatabaseSeedError> {
    info!("Seeding barangays...");

    let mut executor = db
        .acquire()
        .await
        .map_err(|e| crate::database::DatabaseSeedError::DbError(e))?;

    let city_map = get_city_map(&mut executor).await;
    let municipality_map = get_municipality_map(&mut executor).await;
    // TODO: add district mapping

    let barangays_json = include_str!("../data/json/barangays.json");
    let barangays_data = serde_json::from_str::<Vec<BarangayData>>(barangays_json)
        .map_err(|e| crate::database::DatabaseSeedError::Serialization(e))?;

    let barangays: Vec<Barangay> = barangays_data
        .iter()
        .map(|barangay| {
            let municipality_code = &barangay.correspondence_code[0..8];
            let municipality_id = municipality_map.get(municipality_code).cloned();

            let city_code = &barangay.correspondence_code[0..5];
            let city_id = {
                let id = city_map.get(city_code).cloned();

                match id {
                    None => city_map.get(&barangay.code[0..5]).cloned(),
                    id => id,
                }
            };

            let barangay = Barangay::builder()
                .id(uuid_now())
                .name(barangay.name.to_owned())
                .population(0)
                .code(barangay.code.to_owned())
                .correspondence_code(barangay.correspondence_code.to_owned())
                .urban_rural(barangay.urban_rural.to_owned())
                .maybe_city_id(city_id)
                .maybe_municipality_id(municipality_id)
                // TODO: add district_id
                .created_at(datetime_utc_now())
                .updated_at(datetime_utc_now())
                .build();

            barangay
        })
        .collect::<Vec<_>>();

    Barangay::insert_batch(&mut executor, &barangays, 1000)
        .await
        .map_err(|e| crate::database::DatabaseSeedError::DbError(e))?;

    info!("Added {} barangays to database", barangays.len());

    Ok(())
}
