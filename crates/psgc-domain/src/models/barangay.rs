use chrono::Utc;

use crate::models::{city::City, district::District, municipality::Municipality};

#[allow(dead_code)]
#[derive(Debug, bon::Builder)]
pub struct Barangay {
    pub id: uuid::Uuid,
    pub code: String,
    pub correspondence_code: String,
    pub name: String,
    pub population: u64,
    pub city_id: Option<uuid::Uuid>,
    pub municipality_id: Option<uuid::Uuid>,
    pub district_id: Option<uuid::Uuid>,
    pub urban_rural: String,
    pub city: Option<City>,
    pub municipality: Option<Municipality>,
    pub district: Option<District>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
