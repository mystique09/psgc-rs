use chrono::Utc;

use crate::models::{province::Province, region::Region};

#[allow(dead_code)]
#[derive(Debug, bon::Builder)]
pub struct City {
    pub id: uuid::Uuid,
    pub code: String,
    pub correspondence_code: String,
    pub name: String,
    pub population: u64,
    pub city_class: String,
    pub income_class: String,
    pub region_id: Option<uuid::Uuid>,
    pub province_id: Option<uuid::Uuid>,
    pub region: Option<Region>,
    pub province: Option<Province>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
