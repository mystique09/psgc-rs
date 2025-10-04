use chrono::Utc;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, bon::Builder)]
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
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

rbatis::crud!(Barangay {});
