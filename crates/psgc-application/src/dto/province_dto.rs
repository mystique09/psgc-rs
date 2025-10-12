use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvinceDTO {
    pub id: uuid::Uuid,
    pub code: String,
    pub correspondence_code: String,
    pub name: String,
    pub population: u64,
    pub income_class: String,
    pub region_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<psgc_domain::models::province::Province> for ProvinceDTO {
    fn from(province: psgc_domain::models::province::Province) -> Self {
        Self {
            id: province.id,
            code: province.code,
            correspondence_code: province.correspondence_code,
            name: province.name,
            population: province.population,
            income_class: province.income_class,
            region_id: province.region_id,
            created_at: province.created_at,
            updated_at: province.updated_at,
        }
    }
}
