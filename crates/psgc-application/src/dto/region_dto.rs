use serde::Serialize;

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct RegionDTO {
    pub id: uuid::Uuid,
    pub code: String,
    pub correspondence_code: String,
    pub name: String,
    pub designation: String,
    pub population: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<psgc_domain::models::region::Region> for RegionDTO {
    fn from(region: psgc_domain::models::region::Region) -> Self {
        Self {
            id: region.id,
            code: region.code,
            correspondence_code: region.correspondence_code,
            name: region.name,
            designation: region.designation,
            population: region.population,
            created_at: region.created_at,
            updated_at: region.updated_at,
        }
    }
}
