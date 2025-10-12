use std::future;

use crate::{
    errors::RepositoryError,
    models::{
        PaginateResult, barangay::Barangay, city::City, district::District,
        municipality::Municipality,
    },
};

pub trait DistrictRepository: Send + Sync + 'static {
    fn find_by_code(
        &self,
        code: &str,
    ) -> impl future::Future<Output = Result<District, RepositoryError>>;
    fn list_all(
        &self,
        page: u64,
        limit: u64,
    ) -> impl future::Future<Output = Result<PaginateResult<District>, RepositoryError>>;
    fn list_by_region_id(
        &self,
        region_id: &uuid::Uuid,
    ) -> impl future::Future<Output = Result<Vec<District>, RepositoryError>>;
    fn list_by_province_id(
        &self,
        province_id: &uuid::Uuid,
    ) -> impl future::Future<Output = Result<Vec<District>, RepositoryError>>;
    fn list_cities_by_district_id(
        &self,
        district_id: &uuid::Uuid,
    ) -> impl future::Future<Output = Result<Vec<City>, RepositoryError>>;
    fn list_municipalities_by_district_id(
        &self,
        district_id: &uuid::Uuid,
    ) -> impl future::Future<Output = Result<Vec<Municipality>, RepositoryError>>;
    fn list_barangays_by_district_id(
        &self,
        district_id: &uuid::Uuid,
    ) -> impl future::Future<Output = Result<Vec<Barangay>, RepositoryError>>;
}
