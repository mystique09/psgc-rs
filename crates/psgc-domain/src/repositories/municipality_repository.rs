use std::future;

use crate::{
    errors::RepositoryError,
    models::{PaginateResult, barangay::Barangay, municipality::Municipality},
};

pub trait MunicipalityRepository: Send + Sync + 'static {
    fn find_by_code(
        &self,
        code: &str,
    ) -> impl future::Future<Output = Result<Municipality, RepositoryError>>;
    fn list_all(
        &self,
        page: u64,
        limit: u64,
    ) -> impl future::Future<Output = Result<PaginateResult<Municipality>, RepositoryError>>;
    fn list_by_region_code(
        &self,
        region_code: &str,
    ) -> impl future::Future<Output = Result<Vec<Municipality>, RepositoryError>>;
    fn list_by_province_code(
        &self,
        province_code: &str,
    ) -> impl future::Future<Output = Result<Vec<Municipality>, RepositoryError>>;
    fn list_by_district_code(
        &self,
        district_code: &str,
    ) -> impl future::Future<Output = Result<Vec<Municipality>, RepositoryError>>;
    fn list_barangays(
        &self,
        municipality_code: &str,
    ) -> impl future::Future<Output = Result<Vec<Barangay>, RepositoryError>>;
}
