use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Default, Serialize, Deserialize, bon::Builder)]
pub struct District {
    pub id: rbatis::rbdc::Uuid,
    pub code: String,
    pub correspondence_code: String,
    pub name: String,
    pub population: u64,
    pub region_id: Option<rbatis::rbdc::Uuid>,
    pub province_id: Option<rbatis::rbdc::Uuid>,
    pub created_at: rbatis::rbdc::DateTime,
    pub updated_at: rbatis::rbdc::DateTime,
}

rbatis::crud!(District {}, "districts");
rbatis::impl_select_page!(District {list_districts() => ""}, "districts");
rbatis::impl_select!(District {list_districts_by_region_id(region_id: &rbatis::rbdc::Uuid) => "`where region_id = #{region_id}`"}, "districts");
rbatis::impl_select!(District {list_districts_by_province_id(province_id: &rbatis::rbdc::Uuid) => "`where province_id = #{province_id}`"}, "districts");
rbatis::impl_select!(District {select_by_code(code: &str) -> Option => "`where code = #{code} limit 1`"}, "districts");

// TODO: add seeder
