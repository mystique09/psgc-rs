use chrono::Utc;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, bon::Builder)]
pub struct District {
    id: uuid::Uuid,
    code: String,
    correspondence_code: String,
    name: String,
    population: u64,
    region_id: Option<uuid::Uuid>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

rbatis::crud!(District {});
