use std::future;

use crate::{
    errors::RepositoryError,
    models::{PaginateResult, barangay::Barangay},
};

pub trait BarangayRepository: Send + Sync + 'static {
    fn find_by_code(
        &self,
        code: &str,
    ) -> impl future::Future<Output = Result<Barangay, RepositoryError>>;
    fn list_all(
        &self,
        page: u64,
        limit: u64,
    ) -> impl future::Future<Output = Result<PaginateResult<Barangay>, RepositoryError>>;
    fn list_by_city_code(
        &self,
        code: &str,
    ) -> impl future::Future<Output = Result<Vec<Barangay>, RepositoryError>>;
    fn list_by_municipality_code(
        &self,
        code: &str,
    ) -> impl future::Future<Output = Result<Vec<Barangay>, RepositoryError>>;
    fn list_by_district_code(
        &self,
        code: &str,
    ) -> impl future::Future<Output = Result<Vec<Barangay>, RepositoryError>>;
}
