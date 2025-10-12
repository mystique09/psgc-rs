use std::sync::Arc;

use psgc_domain::{
    errors::RepositoryError,
    models::{PaginateResult, barangay::Barangay, city::City},
    repositories::city_repository::CityRepository,
};
use rbatis::{PageRequest, RBatis};

use crate::database::{generators::PageExt, models};

pub struct PgCityRepository {
    db: Arc<RBatis>,
}

impl PgCityRepository {
    pub fn new(db: Arc<RBatis>) -> Self {
        Self { db }
    }
}

#[allow(unused)]
impl CityRepository for PgCityRepository {
    async fn find_by_code(&self, code: &str) -> Result<City, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let city = models::city::City::select_by_code(&mut executor, code)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?
            .ok_or(RepositoryError::NotFound)?;

        Ok(city.into())
    }

    async fn list_all(
        &self,
        page: u64,
        limit: u64,
    ) -> Result<PaginateResult<City>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let cities = models::city::City::list_cities(&mut executor, &PageRequest::new(page, limit))
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(cities.into_domain::<City>())
    }

    async fn list_by_region_code(&self, code: &str) -> Result<Vec<City>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let cities = models::city::City::list_cities_by_region_code(&mut executor, code)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(cities.into_iter().map(|c| c.into()).collect())
    }

    async fn list_by_province_code(&self, code: &str) -> Result<Vec<City>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let cities = models::city::City::list_cities_by_province_code(&mut executor, code)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(cities.into_iter().map(|c| c.into()).collect())
    }

    async fn list_barangays(&self, codename: &str) -> Result<Vec<Barangay>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let barangays =
            models::barangay::Barangay::list_barangays_by_city_code(&mut executor, &codename)
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(barangays.into_iter().map(|b| b.into()).collect())
    }
}
