use chrono::Utc;

#[allow(dead_code)]
#[derive(Debug, bon::Builder)]
pub struct Region {
    id: uuid::Uuid,
    code: String,
    correspondence_code: String,
    name: String,
    designation: String,
    population: u64,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}
