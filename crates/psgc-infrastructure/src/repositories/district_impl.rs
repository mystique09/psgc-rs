use std::sync::Arc;

use psgc_domain::{
    errors::RepositoryError,
    models::{PaginateResult, city::City, district::District, municipality::Municipality},
    repositories::district_repository::DistrictRepository,
};
use rbatis::{PageRequest, RBatis};

use crate::database::{generators::PageExt, models};

pub struct PgDistrictRepository {
    db: Arc<RBatis>,
}

impl PgDistrictRepository {
    pub fn new(db: Arc<RBatis>) -> Self {
        Self { db }
    }
}

#[allow(unused)]
impl DistrictRepository for PgDistrictRepository {
    async fn find_by_code(&self, code: &str) -> Result<District, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let district = models::district::District::select_by_code(&mut executor, code)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?
            .ok_or(RepositoryError::NotFound)?;

        Ok(district.into())
    }

    async fn list_all(
        &self,
        page: u64,
        limit: u64,
    ) -> Result<PaginateResult<District>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let districts = models::district::District::list_districts(
            &mut executor,
            &PageRequest::new(page, limit),
        )
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(districts.into_domain::<District>())
    }

    async fn list_by_region_code(&self, code: &str) -> Result<Vec<District>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let districts =
            models::district::District::list_districts_by_region_code(&mut executor, code)
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(districts.into_iter().map(|d| d.into()).collect())
    }

    async fn list_by_province_code(&self, code: &str) -> Result<Vec<District>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let districts =
            models::district::District::list_districts_by_province_code(&mut executor, code)
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(districts.into_iter().map(|d| d.into()).collect())
    }

    async fn list_cities(&self, code: &str) -> Result<Vec<City>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let cities = models::city::City::list_cities_by_district_code(&mut executor, code)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(cities.into_iter().map(|c| c.into()).collect())
    }

    async fn list_municipalities(&self, code: &str) -> Result<Vec<Municipality>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let municipalities =
            models::municipality::Municipality::list_municipalities_by_district_code(
                &mut executor,
                code,
            )
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(municipalities.into_iter().map(|m| m.into()).collect())
    }
}
