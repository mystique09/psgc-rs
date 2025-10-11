use std::sync::Arc;

use psgc_domain::{
    errors::RepositoryError,
    models::{
        PaginateResult, barangay::Barangay, city::City, municipality::Municipality,
        province::Province, region::Region,
    },
    repositories::region_repository::RegionRepository,
};
use rbatis::{PageRequest, RBatis};

use crate::database::{generators::PageExt, models};

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
        page: u64,
        limit: u64,
    ) -> Result<Vec<Province>, RepositoryError> {
        todo!()
    }

    async fn list_cities(&self, page: u64, limit: u64) -> Result<Vec<City>, RepositoryError> {
        todo!()
    }

    async fn list_municipalities(
        &self,
        page: u64,
        limit: u64,
    ) -> Result<Vec<Municipality>, RepositoryError> {
        todo!()
    }

    async fn list_barangays(
        &self,
        page: u64,
        limit: u64,
    ) -> Result<Vec<Barangay>, RepositoryError> {
        todo!()
    }
}
