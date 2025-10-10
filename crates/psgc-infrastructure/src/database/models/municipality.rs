use crate::database::{
    DatabaseSeedError,
    generators::{datetime_utc_now, uuid_now},
    helpers::{get_province_map, get_province_map_2, get_region_map},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, bon::Builder)]
pub struct Municipality {
    pub id: rbatis::rbdc::Uuid,
    pub code: String,
    pub correspondence_code: String,
    pub name: String,
    pub population: u64,
    pub income_class: String,
    pub region_id: Option<rbatis::rbdc::Uuid>,
    pub province_id: Option<rbatis::rbdc::Uuid>,
    pub district_id: Option<rbatis::rbdc::Uuid>,
    pub sub_municipality_id: Option<rbatis::rbdc::Uuid>,
    pub created_at: rbatis::rbdc::DateTime,
    pub updated_at: rbatis::rbdc::DateTime,
}

#[derive(Debug, Serialize, Deserialize, bon::Builder)]
struct MunicipalityData {
    #[serde(rename = "psgc10DigitCode")]
    correspondence_code: String,
    name: String,
    #[serde(rename = "code")]
    code: String,
    #[serde(rename = "incomeClassification")]
    income_class: String,
}

rbatis::crud!(Municipality {}, "municipalities");

pub async fn seed_municipalities(db: &rbatis::RBatis) -> Result<(), DatabaseSeedError> {
    let mut executor = db
        .acquire()
        .await
        .map_err(|e| crate::database::DatabaseSeedError::DbError(e))?;

    let region_map = get_region_map(&mut executor).await;
    let province_map = get_province_map(&mut executor).await;
    let province_map_by_code = get_province_map_2(&mut executor).await;

    let municipalities_json = include_str!("../data/json/municipalities.json");
    let municipalities_data = serde_json::from_str::<Vec<MunicipalityData>>(municipalities_json)
        .map_err(|e| crate::database::DatabaseSeedError::Serialization(e))?;

    let municipalities: Vec<Municipality> = municipalities_data
        .iter()
        .map(|m| {
            let region_code = &m.code[0..2];
            let region_id = region_map.get(region_code).cloned();

            let province_code = &m.correspondence_code[0..4];
            let province_id = {
                let id = province_map.get(province_code).cloned();

                match id {
                    Some(id) => Some(id),
                    None => province_map_by_code.get(&m.code[0..4]).cloned(),
                }
            };

            let municipality = Municipality::builder()
                .id(uuid_now())
                .name(m.name.to_owned())
                .code(m.code.to_owned())
                .correspondence_code(m.correspondence_code.to_owned())
                .population(0)
                .income_class(m.income_class.to_owned())
                .maybe_region_id(region_id)
                .maybe_province_id(province_id)
                // TODO: add district_id
                .created_at(datetime_utc_now())
                .updated_at(datetime_utc_now())
                .build();

            municipality
        })
        .collect::<Vec<_>>();

    Municipality::insert_batch(&mut executor, &municipalities, 100)
        .await
        .map_err(|e| crate::database::DatabaseSeedError::DbError(e))?;

    Ok(())
}
