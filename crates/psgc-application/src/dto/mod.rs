use serde::{Deserialize, Serialize};

pub mod barangay_dto;
pub mod city_dto;
pub mod district_dto;
pub mod municipality_dto;
pub mod province_dto;
pub mod region_dto;

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginateResponseDTO<T: Serialize> {
    pub records: Vec<T>,
    pub total: u64,
    pub page_no: u64,
    pub page_size: u64,
}

impl<T, U> From<psgc_domain::models::PaginateResult<T>> for PaginateResponseDTO<U>
where
    T: Into<U>,
    U: Serialize,
{
    fn from(domain_result: psgc_domain::models::PaginateResult<T>) -> Self {
        Self {
            records: domain_result
                .records
                .into_iter()
                .map(|r| r.into())
                .collect(),
            total: domain_result.total,
            page_no: domain_result.page_no,
            page_size: domain_result.page_size,
        }
    }
}
