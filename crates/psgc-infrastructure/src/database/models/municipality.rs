use chrono::Utc;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Default, Serialize, Deserialize, bon::Builder)]
pub struct Municipality {
    id: uuid::Uuid,
    code: String,
    correspondence_code: String,
    name: String,
    population: u64,
    income_class: String,
    region_id: Option<uuid::Uuid>,
    province_id: Option<uuid::Uuid>,
    city_id: Option<uuid::Uuid>,
    district_id: Option<uuid::Uuid>,
    sub_municipality_id: Option<uuid::Uuid>,
    barangay_id: Option<uuid::Uuid>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

rbatis::crud!(Municipality {}, "municipalities");
