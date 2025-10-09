use chrono::Utc;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Default, Serialize, Deserialize, bon::Builder)]
pub struct Province {
    id: uuid::Uuid,
    code: String,
    correspondence_code: String,
    name: String,
    population: u64,
    income_class: String,
    region_id: Option<uuid::Uuid>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

rbatis::crud!(Province {});
