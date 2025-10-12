use serde::Serialize;

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct MunicipalityDTO {
    pub id: uuid::Uuid,
    pub code: String,
    pub correspondence_code: String,
    pub name: String,
    pub population: u64,
    pub income_class: String,
    pub region_id: Option<uuid::Uuid>,
    pub province_id: Option<uuid::Uuid>,
    pub district_id: Option<uuid::Uuid>,
    pub sub_municipality_id: Option<uuid::Uuid>,
    pub barangay_id: Option<uuid::Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<psgc_domain::models::municipality::Municipality> for MunicipalityDTO {
    fn from(municipality: psgc_domain::models::municipality::Municipality) -> Self {
        Self {
            id: municipality.id,
            code: municipality.code,
            correspondence_code: municipality.correspondence_code,
            name: municipality.name,
            population: municipality.population,
            income_class: municipality.income_class,
            region_id: municipality.region_id,
            province_id: municipality.province_id,
            district_id: municipality.district_id,
            sub_municipality_id: municipality.sub_municipality_id,
            barangay_id: municipality.barangay_id,
            created_at: municipality.created_at,
            updated_at: municipality.updated_at,
        }
    }
}
