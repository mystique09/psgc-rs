use std::sync::Arc;

use psgc_domain::{
    errors::RepositoryError,
    models::{PaginateResult, barangay::Barangay, city::City},
    repositories::city_repository::CityRepository,
};
use rbatis::{PageRequest, RBatis};

use crate::database::{
    generators::{PageExt, UuidExt},
    models,
};

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

    async fn list_by_region_id(
        &self,
        region_id: &uuid::Uuid,
    ) -> Result<Vec<City>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let db_region_id = region_id.into_db();
        let cities = models::city::City::list_cities_by_region_id(&mut executor, &db_region_id)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(cities.into_iter().map(|c| c.into()).collect())
    }

    async fn list_by_province_id(
        &self,
        province_id: &uuid::Uuid,
    ) -> Result<Vec<City>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let db_province_id = province_id.into_db();
        let cities = models::city::City::list_cities_by_province_id(&mut executor, &db_province_id)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(cities.into_iter().map(|c| c.into()).collect())
    }

    async fn list_barangays_by_city_id(
        &self,
        city_id: &uuid::Uuid,
    ) -> Result<Vec<Barangay>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let db_city_id = city_id.into_db();
        let barangays =
            models::barangay::Barangay::list_barangays_by_city_id(&mut executor, &db_city_id)
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(barangays.into_iter().map(|b| b.into()).collect())
    }
}
