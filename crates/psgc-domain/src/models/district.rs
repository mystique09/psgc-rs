use chrono::Utc;

use crate::models::region::Region;

#[allow(dead_code)]
#[derive(Debug, bon::Builder)]
pub struct District {
    id: uuid::Uuid,
    code: String,
    name: String,
    population: u64,
    region_id: uuid::Uuid,
    region: Option<Region>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}
