use std::future;

use crate::{
    errors::RepositoryError,
    models::{PaginateResult, barangay::Barangay, city::City},
};

pub trait CityRepository: Send + Sync + 'static {
    fn find_by_code(
        &self,
        code: &str,
    ) -> impl future::Future<Output = Result<City, RepositoryError>>;
    fn list_all(
        &self,
        page: u64,
        limit: u64,
    ) -> impl future::Future<Output = Result<PaginateResult<City>, RepositoryError>>;
    fn list_by_region_id(
        &self,
        region_id: &uuid::Uuid,
    ) -> impl future::Future<Output = Result<Vec<City>, RepositoryError>>;
    fn list_by_province_id(
        &self,
        province_id: &uuid::Uuid,
    ) -> impl future::Future<Output = Result<Vec<City>, RepositoryError>>;
    fn list_barangays_by_city_id(
        &self,
        city_id: &uuid::Uuid,
    ) -> impl future::Future<Output = Result<Vec<Barangay>, RepositoryError>>;
}
