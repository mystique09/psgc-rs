use std::sync::Arc;

use psgc_domain::{
    errors::RepositoryError,
    models::{PaginateResult, city::City, municipality::Municipality, province::Province},
    repositories::province_repository::ProvinceRepository,
};
use rbatis::{PageRequest, RBatis};

use crate::database::{generators::PageExt, models};

pub struct PgProvinceRepository {
    db: Arc<RBatis>,
}

impl PgProvinceRepository {
    pub fn new(db: Arc<RBatis>) -> Self {
        Self { db }
    }
}

#[allow(unused)]
impl ProvinceRepository for PgProvinceRepository {
    async fn find_by_code(&self, code: &str) -> Result<Province, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let province = models::province::Province::select_by_code(&mut executor, code)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?
            .ok_or(RepositoryError::NotFound)?;

        Ok(province.into())
    }

    async fn list_all(
        &self,
        page: u64,
        limit: u64,
    ) -> Result<PaginateResult<Province>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let provinces = models::province::Province::list_provinces(
            &mut executor,
            &PageRequest::new(page, limit),
        )
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(provinces.into_domain::<Province>())
    }

    async fn list_by_region_code(&self, code: &str) -> Result<Vec<Province>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let provinces =
            models::province::Province::list_provinces_by_region_code(&mut executor, code)
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(provinces.into_iter().map(|p| p.into()).collect())
    }

    async fn list_cities(&self, code: &str) -> Result<Vec<City>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let cities = models::city::City::list_cities_by_province_code(&mut executor, code)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(cities.into_iter().map(|c| c.into()).collect())
    }

    async fn list_municipalities(&self, code: &str) -> Result<Vec<Municipality>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let municipalities =
            models::municipality::Municipality::list_municipalities_by_province_code(
                &mut executor,
                code,
            )
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(municipalities.into_iter().map(|m| m.into()).collect())
    }
}
