use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CityDTO {
    pub id: uuid::Uuid,
    pub code: String,
    pub correspondence_code: String,
    pub name: String,
    pub population: u64,
    pub city_class: String,
    pub income_class: String,
    pub region_id: Option<uuid::Uuid>,
    pub province_id: Option<uuid::Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<psgc_domain::models::city::City> for CityDTO {
    fn from(city: psgc_domain::models::city::City) -> Self {
        Self {
            id: city.id,
            code: city.code,
            correspondence_code: city.correspondence_code,
            name: city.name,
            population: city.population,
            city_class: city.city_class,
            income_class: city.income_class,
            region_id: city.region_id,
            province_id: city.province_id,
            created_at: city.created_at,
            updated_at: city.updated_at,
        }
    }
}
