use chrono::Utc;

use crate::models::{city::City, district::District, municipality::Municipality};

#[allow(dead_code)]
#[derive(Debug, bon::Builder)]
pub struct Barangay {
    id: uuid::Uuid,
    code: String,
    correspondence_code: String,
    name: String,
    population: u64,
    city_id: Option<uuid::Uuid>,
    municipality_id: Option<uuid::Uuid>,
    district_id: Option<uuid::Uuid>,
    urban_rural: String,
    city: Option<City>,
    municipality: Option<Municipality>,
    district: Option<District>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}
