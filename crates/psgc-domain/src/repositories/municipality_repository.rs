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
    fn list_by_region_id(
        &self,
        region_id: &uuid::Uuid,
    ) -> impl future::Future<Output = Result<Vec<Municipality>, RepositoryError>>;
    fn list_by_province_id(
        &self,
        province_id: &uuid::Uuid,
    ) -> impl future::Future<Output = Result<Vec<Municipality>, RepositoryError>>;
    fn list_by_district_id(
        &self,
        district_id: &uuid::Uuid,
    ) -> impl future::Future<Output = Result<Vec<Municipality>, RepositoryError>>;
    fn list_barangays_by_municipality_id(
        &self,
        municipality_id: &uuid::Uuid,
    ) -> impl future::Future<Output = Result<Vec<Barangay>, RepositoryError>>;
}
