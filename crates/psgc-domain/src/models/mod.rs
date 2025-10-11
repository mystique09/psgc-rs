pub mod barangay;
pub mod city;
pub mod district;
pub mod municipality;
pub mod province;
pub mod region;

#[derive(Debug, bon::Builder)]
pub struct PaginateResult<T> {
    pub records: Vec<T>,
    /// total num
    pub total: u64,
    /// current page index
    pub page_no: u64,
    /// default 10
    pub page_size: u64,
}
