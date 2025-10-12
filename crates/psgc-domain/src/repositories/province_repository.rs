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
    fn list_by_region_code(
        &self,
        region_code: &str,
    ) -> impl future::Future<Output = Result<Vec<Province>, RepositoryError>>;
    fn list_cities(
        &self,
        province_code: &str,
    ) -> impl future::Future<Output = Result<Vec<City>, RepositoryError>>;
    fn list_municipalities(
        &self,
        province_code: &str,
    ) -> impl future::Future<Output = Result<Vec<Municipality>, RepositoryError>>;
}
