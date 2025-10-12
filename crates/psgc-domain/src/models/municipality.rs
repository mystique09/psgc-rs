use chrono::Utc;

use crate::models::{city::City, district::District, province::Province, region::Region};

#[allow(dead_code)]
#[derive(Debug, bon::Builder)]
pub struct Municipality {
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
    pub region: Option<Region>,
    pub province: Option<Province>,
    pub city: Option<City>,
    pub district: Option<District>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
