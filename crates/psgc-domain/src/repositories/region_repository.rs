use std::future;

use crate::{
    errors::RepositoryError,
    models::{
        PaginateResult, barangay::Barangay, city::City, municipality::Municipality,
        province::Province, region::Region,
    },
};

pub trait RegionRepository: Send + Sync + 'static {
    fn find_by_codename(
        &self,
        code_name: &str,
    ) -> impl future::Future<Output = Result<Region, RepositoryError>>;
    fn list_all(
        &self,
        page: u64,
        limit: u64,
    ) -> impl future::Future<Output = Result<PaginateResult<Region>, RepositoryError>>;
    fn list_provinces(
        &self,
        page: u64,
        limit: u64,
    ) -> impl future::Future<Output = Result<Vec<Province>, RepositoryError>>;
    fn list_cities(
        &self,
        page: u64,
        limit: u64,
    ) -> impl future::Future<Output = Result<Vec<City>, RepositoryError>>;
    fn list_municipalities(
        &self,
        page: u64,
        limit: u64,
    ) -> impl future::Future<Output = Result<Vec<Municipality>, RepositoryError>>;
    fn list_barangays(
        &self,
        page: u64,
        limit: u64,
    ) -> impl future::Future<Output = Result<Vec<Barangay>, RepositoryError>>;
}
