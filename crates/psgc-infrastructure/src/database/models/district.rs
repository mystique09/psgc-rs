use crate::database::generators::{DateTimeUtcExt, RBatisUuidExt};
use rbatis::executor::Executor;
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

impl District {
    #[rbatis::py_sql(
        "SELECT d.* FROM districts d LEFT JOIN regions r ON d.region_id = r.id WHERE r.code = #{code}"
    )]
    async fn list_districts_by_region_code(rb: &dyn Executor, code: &str) -> Vec<District> {}

    #[rbatis::py_sql(
        "SELECT d.* FROM districts d LEFT JOIN provinces p ON d.province_id = p.id WHERE p.code = #{code}"
    )]
    async fn list_districts_by_province_code(rb: &dyn Executor, code: &str) -> Vec<District> {}
}

rbatis::crud!(District {}, "districts");
rbatis::impl_select_page!(District {list_districts() => ""}, "districts");

rbatis::impl_select!(District {select_by_code(code: &str) -> Option => "`where code = #{code} limit 1`"}, "districts");

impl From<District> for psgc_domain::models::district::District {
    fn from(value: District) -> Self {
        let region_id = value
            .region_id
            .map(|id| id.inner())
            .unwrap_or_else(uuid::Uuid::new_v4);

        Self::builder()
            .id(value.id.inner())
            .name(value.name)
            .code(value.code)
            .correspondence_code(value.correspondence_code)
            .population(value.population)
            .region_id(region_id)
            .created_at(value.created_at.inner())
            .updated_at(value.updated_at.inner())
            .build()
    }
}

// TODO: add seeder
