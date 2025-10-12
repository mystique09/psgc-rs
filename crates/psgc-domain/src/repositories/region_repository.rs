use std::future;

use crate::{
    errors::RepositoryError,
    models::{
        PaginateResult, city::City, municipality::Municipality, province::Province, region::Region,
    },
};

pub trait RegionRepository: Send + Sync + 'static {
    fn find_by_code(
        &self,
        code: &str,
    ) -> impl future::Future<Output = Result<Region, RepositoryError>>;
    fn list_all(
        &self,
        region_id: &uuid::Uuid,
        page: u64,
        limit: u64,
    ) -> impl future::Future<Output = Result<PaginateResult<Region>, RepositoryError>>;
    fn list_provinces(
        &self,
        code: &str,
    ) -> impl future::Future<Output = Result<Vec<Province>, RepositoryError>>;
    fn list_cities(
        &self,
        code: &str,
    ) -> impl future::Future<Output = Result<Vec<City>, RepositoryError>>;
    fn list_municipalities(
        &self,
        code: &str,
    ) -> impl future::Future<Output = Result<Vec<Municipality>, RepositoryError>>;
}
