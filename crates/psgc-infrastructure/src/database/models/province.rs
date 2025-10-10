use crate::database::{generators::datetime_utc_now, helpers::get_region_map};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Default, Serialize, Deserialize, bon::Builder)]
pub struct Province {
    pub id: rbatis::rbdc::Uuid,
    pub code: String,
    pub correspondence_code: String,
    pub name: String,
    pub population: u64,
    pub income_class: String,
    pub region_id: Option<rbatis::rbdc::Uuid>,
    pub created_at: rbatis::rbdc::DateTime,
    pub updated_at: rbatis::rbdc::DateTime,
}

rbatis::crud!(Province {}, "provinces");

#[derive(Debug, Serialize, Deserialize, bon::Builder)]
struct ProvinceData {
    #[serde(rename = "psgc10DigitCode")]
    correspondence_code: String,
    name: String,
    #[serde(rename = "code")]
    code: String,
    #[serde(rename = "geographicLevel")]
    geographic_level: String,
    #[serde(rename = "oldName")]
    old_name: String,
    #[serde(rename = "cityClass")]
    city_class: String,
    #[serde(rename = "incomeClassification")]
    income_classification: String,
    #[serde(rename = "urbanRural")]
    urban_rural: String,
}

pub async fn seed_provinces(db: &rbatis::RBatis) -> Result<(), crate::database::DatabaseSeedError> {
    println!("Seeding provinces...");

    let mut executor = db
        .acquire()
        .await
        .map_err(|e| crate::database::DatabaseSeedError::DbError(e))?;

    let region_map = get_region_map(&mut executor).await;

    let provinces_json = include_str!("../data/json/provinces.json");
    let provinces_data = serde_json::from_str::<Vec<ProvinceData>>(provinces_json)
        .map_err(|e| crate::database::DatabaseSeedError::Serialization(e))?;

    let provinces: Vec<Province> = provinces_data
        .iter()
        .filter_map(|p| {
            // Extract 2-character region code from province
            let region_code = &p.code[0..2];
            let region_id = region_map.get(region_code).cloned();

            if region_id.is_none() {
                println!(
                    "WARN: Province {} ({}) - No region found for code {}",
                    p.name, p.code, region_code
                );
            }

            Some(
                Province::builder()
                    .id(crate::database::generators::uuid_now())
                    .code(p.code.clone())
                    .correspondence_code(p.correspondence_code.clone())
                    .name(p.name.clone())
                    .population(0) // Population not available in province data
                    .income_class(p.income_classification.clone())
                    .maybe_region_id(region_id)
                    .created_at(datetime_utc_now())
                    .updated_at(datetime_utc_now())
                    .build(),
            )
        })
        .collect();

    Province::insert_batch(db, &provinces, 100)
        .await
        .map_err(|e| crate::database::DatabaseSeedError::DbError(e))?;

    println!("Added {} provinces to database", provinces.len());
    Ok(())
}
