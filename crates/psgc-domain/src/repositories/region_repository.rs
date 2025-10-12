use std::future;

use crate::{
    errors::RepositoryError,
    models::{
        PaginateResult, city::City, municipality::Municipality, province::Province, region::Region,
    },
};

pub trait RegionRepository: Send + Sync + 'static {
    fn find_by_codename(
        &self,
        code_name: &str,
    ) -> impl future::Future<Output = Result<Region, RepositoryError>>;
    fn list_all(
        &self,
        region_id: &uuid::Uuid,
        page: u64,
        limit: u64,
    ) -> impl future::Future<Output = Result<PaginateResult<Region>, RepositoryError>>;
    fn list_provinces(
        &self,
        region_id: &uuid::Uuid,
    ) -> impl future::Future<Output = Result<Vec<Province>, RepositoryError>>;
    fn list_cities(
        &self,
        region_id: &uuid::Uuid,
    ) -> impl future::Future<Output = Result<Vec<City>, RepositoryError>>;
    fn list_municipalities(
        &self,
        region_id: &uuid::Uuid,
    ) -> impl future::Future<Output = Result<Vec<Municipality>, RepositoryError>>;
}
