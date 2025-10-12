use chrono::Utc;

use crate::models::region::Region;

#[allow(dead_code)]
#[derive(Debug, bon::Builder)]
pub struct District {
    pub id: uuid::Uuid,
    pub code: String,
    pub correspondence_code: String,
    pub name: String,
    pub population: u64,
    pub region_id: uuid::Uuid,
    pub region: Option<Region>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
