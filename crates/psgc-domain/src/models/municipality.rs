use chrono::Utc;

use crate::models::{
    city::City, district::District, province::Province, region::Region,
    sub_municipality::SubMunicipality,
};

#[allow(dead_code)]
#[derive(Debug, bon::Builder)]
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
    region: Option<Region>,
    province: Option<Province>,
    city: Option<City>,
    district: Option<District>,
    sub_municipality: Option<SubMunicipality>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}
