use chrono::Utc;

use crate::models::region::Region;

#[allow(dead_code)]
#[derive(Debug, bon::Builder)]
pub struct Province {
    id: uuid::Uuid,
    code: String,
    correspondence_code: String,
    name: String,
    population: u64,
    region_id: uuid::Uuid,
    income_class: String,
    region: Option<Region>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}
