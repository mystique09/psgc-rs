use chrono::Utc;

use crate::models::city::City;

#[allow(dead_code)]
#[derive(Debug, bon::Builder)]
pub struct SubMunicipality {
    id: uuid::Uuid,
    code: String,
    correspondence_code: String,
    name: String,
    population: u64,
    city_id: uuid::Uuid,
    city: Option<City>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}
