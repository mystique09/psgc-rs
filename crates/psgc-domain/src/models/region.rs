use chrono::Utc;

#[allow(dead_code)]
#[derive(Debug, bon::Builder)]
pub struct Region {
    pub id: uuid::Uuid,
    pub code: String,
    pub correspondence_code: String,
    pub name: String,
    pub designation: String,
    pub population: u64,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
