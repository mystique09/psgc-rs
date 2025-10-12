use std::future;

use crate::{
    errors::RepositoryError,
    models::{PaginateResult, city::City, municipality::Municipality, province::Province},
};

pub trait ProvinceRepository: Send + Sync + 'static {
    fn find_by_code(
        &self,
        code: &str,
    ) -> impl future::Future<Output = Result<Province, RepositoryError>>;
    fn list_all(
        &self,
        page: u64,
        limit: u64,
    ) -> impl future::Future<Output = Result<PaginateResult<Province>, RepositoryError>>;
    fn list_by_region_id(
        &self,
        region_id: &uuid::Uuid,
    ) -> impl future::Future<Output = Result<Vec<Province>, RepositoryError>>;
    fn list_cities_by_province_id(
        &self,
        province_id: &uuid::Uuid,
    ) -> impl future::Future<Output = Result<Vec<City>, RepositoryError>>;
    fn list_municipalities_by_province_id(
        &self,
        province_id: &uuid::Uuid,
    ) -> impl future::Future<Output = Result<Vec<Municipality>, RepositoryError>>;
}
