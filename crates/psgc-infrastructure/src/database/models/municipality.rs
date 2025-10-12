use crate::database::{
    DatabaseSeedError,
    generators::{DateTimeUtcExt, RBatisUuidExt, datetime_utc_now, uuid_now},
    helpers::{get_province_map, get_province_map_2, get_region_map},
};
use serde::{Deserialize, Serialize};
use tracing::info;

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
rbatis::impl_select_page!(Municipality {list_municipalities() => ""}, "municipalities");
rbatis::impl_select!(Municipality {list_municipalities_by_region_code(code: &str) => "`LEFT JOIN regions r ON municipalities.region_id = r.id WHERE r.code = #{code}`"}, "municipalities");
rbatis::impl_select!(Municipality {list_municipalities_by_province_code(code: &str) => "`LEFT JOIN provinces p ON municipalities.province_id = p.id WHERE p.code = #{code}`"}, "municipalities");
rbatis::impl_select!(Municipality {list_municipalities_by_district_code(code: &str) => "`LEFT JOIN districts d ON municipalities.district_id = d.id WHERE d.code = #{code}`"}, "municipalities");
rbatis::impl_select!(Municipality {select_by_code(code: &str) -> Option => "`where code = #{code} limit 1`"}, "municipalities");

pub async fn seed_municipalities(db: &rbatis::RBatis) -> Result<(), DatabaseSeedError> {
    info!("Seeding municipalities...");

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

    info!("Added {} municipalities to database", municipalities.len());

    Ok(())
}

impl From<Municipality> for psgc_domain::models::municipality::Municipality {
    fn from(value: Municipality) -> Self {
        Self::builder()
            .id(value.id.inner())
            .name(value.name)
            .code(value.code)
            .correspondence_code(value.correspondence_code)
            .population(value.population)
            .income_class(value.income_class)
            .maybe_region_id(value.region_id.map(|id| id.inner()))
            .maybe_province_id(value.province_id.map(|id| id.inner()))
            .maybe_district_id(value.district_id.map(|id| id.inner()))
            .maybe_sub_municipality_id(value.sub_municipality_id.map(|id| id.inner()))
            .created_at(value.created_at.inner())
            .updated_at(value.updated_at.inner())
            .build()
    }
}
