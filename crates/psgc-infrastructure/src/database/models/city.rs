use crate::database::{
    generators::{datetime_utc_now, uuid_now},
    helpers::{get_province_map, get_province_map_2, get_region_map},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, bon::Builder)]
pub struct City {
    pub id: rbatis::rbdc::Uuid,
    pub code: String,
    pub correspondence_code: String,
    pub name: String,
    pub population: u64,
    pub city_class: String,
    pub income_class: String,
    pub region_id: Option<rbatis::rbdc::Uuid>,
    pub province_id: Option<rbatis::rbdc::Uuid>,
    pub created_at: rbatis::rbdc::DateTime,
    pub updated_at: rbatis::rbdc::DateTime,
}

#[derive(Debug, Serialize, Deserialize, bon::Builder)]
struct CityData {
    code: String,
    name: String,
    correspondence_code: String,
    city_class: String,
    income_class: String,
    urban_rural: String,
}

rbatis::crud!(City {}, "cities");

pub async fn seed_cities(db: &rbatis::RBatis) -> Result<(), crate::database::DatabaseSeedError> {
    let mut executor = db
        .acquire()
        .await
        .map_err(|e| crate::database::DatabaseSeedError::DbError(e))?;

    let region_map = get_region_map(&mut executor).await;
    let province_map = get_province_map(&mut executor).await;
    let province_map_by_code = get_province_map_2(&mut executor).await;

    let cities_json = include_str!("../data/json/cities.json");
    let cities_data = serde_json::from_str::<Vec<CityData>>(cities_json)
        .map_err(|e| crate::database::DatabaseSeedError::Serialization(e))?;

    let cities: Vec<City> = cities_data
        .iter()
        .map(|city| {
            let region_code = &city.correspondence_code[0..2];
            let province_code = &city.code[0..4];

            let region_id = region_map.get(region_code).cloned();
            let province_id = {
                let id = province_map.get(province_code).cloned();

                match id {
                    None => match province_map_by_code.get(&city.code[0..4]).cloned() {
                        None => province_map_by_code
                            .get(&city.correspondence_code[0..4])
                            .cloned(),
                        id => id,
                    },
                    _ => id,
                }
            };

            let city = City::builder()
                .id(uuid_now())
                .name(city.name.to_owned())
                .city_class(city.city_class.to_owned())
                .income_class(city.income_class.to_owned())
                .population(0)
                .code(city.code.to_owned())
                .correspondence_code(city.correspondence_code.to_owned())
                .maybe_region_id(region_id)
                .maybe_province_id(province_id)
                .created_at(datetime_utc_now())
                .updated_at(datetime_utc_now())
                .build();

            city
        })
        .collect::<Vec<_>>();

    City::insert_batch(&mut executor, &cities, 100)
        .await
        .map_err(|e| crate::database::DatabaseSeedError::DbError(e))?;

    println!("Added {} cities to database", cities.len());

    Ok(())
}
