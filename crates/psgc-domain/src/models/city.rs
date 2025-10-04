use chrono::Utc;

use crate::models::{province::Province, region::Region};

#[allow(dead_code)]
#[derive(Debug, bon::Builder)]
pub struct City {
    id: uuid::Uuid,
    code: String,
    correspondence_code: String,
    name: String,
    population: u64,
    city_class: String,
    income_class: String,
    region_id: Option<uuid::Uuid>,
    province_id: Option<uuid::Uuid>,
    region: Option<Region>,
    province: Option<Province>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}
