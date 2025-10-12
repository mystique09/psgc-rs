use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DistrictDTO {
    pub id: uuid::Uuid,
    pub code: String,
    pub correspondence_code: String,
    pub name: String,
    pub population: u64,
    pub region_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<psgc_domain::models::district::District> for DistrictDTO {
    fn from(district: psgc_domain::models::district::District) -> Self {
        Self {
            id: district.id,
            code: district.code,
            correspondence_code: district.correspondence_code,
            name: district.name,
            population: district.population,
            region_id: district.region_id,
            created_at: district.created_at,
            updated_at: district.updated_at,
        }
    }
}
