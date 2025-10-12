use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BarangayDTO {
    pub id: uuid::Uuid,
    pub code: String,
    pub correspondence_code: String,
    pub name: String,
    pub population: u64,
    pub city_id: Option<uuid::Uuid>,
    pub municipality_id: Option<uuid::Uuid>,
    pub district_id: Option<uuid::Uuid>,
    pub urban_rural: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<psgc_domain::models::barangay::Barangay> for BarangayDTO {
    fn from(barangay: psgc_domain::models::barangay::Barangay) -> Self {
        Self {
            id: barangay.id,
            code: barangay.code,
            correspondence_code: barangay.correspondence_code,
            name: barangay.name,
            population: barangay.population,
            city_id: barangay.city_id,
            municipality_id: barangay.municipality_id,
            district_id: barangay.district_id,
            urban_rural: barangay.urban_rural,
            created_at: barangay.created_at,
            updated_at: barangay.updated_at,
        }
    }
}
