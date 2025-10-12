use std::sync::Arc;

use psgc_domain::{
    errors::RepositoryError,
    models::{
        PaginateResult, city::City, municipality::Municipality, province::Province, region::Region,
    },
    repositories::region_repository::RegionRepository,
};
use rbatis::{PageRequest, RBatis};

use crate::database::{
    generators::{PageExt, UuidExt},
    models,
};

pub struct PgRegionRepository {
    db: Arc<RBatis>,
}

impl PgRegionRepository {
    pub fn new(db: Arc<RBatis>) -> Self {
        Self { db }
    }
}

#[allow(unused)]
impl RegionRepository for PgRegionRepository {
    async fn find_by_codename(&self, code_name: &str) -> Result<Region, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let region = models::region::Region::select_by_codename(&mut executor, code_name)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?
            .ok_or(RepositoryError::NotFound)?;

        Ok(region.into())
    }

    async fn list_all(
        &self,
        region_id: &uuid::Uuid,
        page: u64,
        limit: u64,
    ) -> Result<PaginateResult<Region>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let regions =
            models::region::Region::list_all(&mut executor, &PageRequest::new(page, limit))
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(regions.into_domain::<Region>())
    }

    async fn list_provinces(
        &self,
        region_id: &uuid::Uuid,
    ) -> Result<Vec<Province>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let db_region_id = region_id.into_db();
        let provinces =
            models::province::Province::list_provinces_by_region_id(&mut executor, &db_region_id)
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(provinces.into_iter().map(|p| p.into()).collect())
    }

    async fn list_cities(&self, region_id: &uuid::Uuid) -> Result<Vec<City>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let db_region_id = region_id.into_db();
        let cities = models::city::City::list_cities_by_region_id(&mut executor, &db_region_id)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(cities.into_iter().map(|c| c.into()).collect())
    }

    async fn list_municipalities(
        &self,
        region_id: &uuid::Uuid,
    ) -> Result<Vec<Municipality>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let db_region_id = region_id.into_db();
        let municipalities = models::municipality::Municipality::list_municipalities_by_region_id(
            &mut executor,
            &db_region_id,
        )
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(municipalities.into_iter().map(|m| m.into()).collect())
    }
}
