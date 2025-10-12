use std::sync::Arc;

use psgc_domain::{
    errors::RepositoryError,
    models::{PaginateResult, barangay::Barangay, municipality::Municipality},
    repositories::municipality_repository::MunicipalityRepository,
};
use rbatis::{PageRequest, RBatis};

use crate::database::{generators::PageExt, models};

pub struct PgMunicipalityRepository {
    db: Arc<RBatis>,
}

impl PgMunicipalityRepository {
    pub fn new(db: Arc<RBatis>) -> Self {
        Self { db }
    }
}

#[allow(unused)]
impl MunicipalityRepository for PgMunicipalityRepository {
    async fn find_by_code(&self, code: &str) -> Result<Municipality, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let municipality = models::municipality::Municipality::select_by_code(&mut executor, code)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?
            .ok_or(RepositoryError::NotFound)?;

        Ok(municipality.into())
    }

    async fn list_all(
        &self,
        page: u64,
        limit: u64,
    ) -> Result<PaginateResult<Municipality>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let municipalities = models::municipality::Municipality::list_municipalities(
            &mut executor,
            &PageRequest::new(page, limit),
        )
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(municipalities.into_domain::<Municipality>())
    }

    async fn list_by_region_code(&self, code: &str) -> Result<Vec<Municipality>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let municipalities =
            models::municipality::Municipality::list_municipalities_by_region_code(
                &mut executor,
                code,
            )
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(municipalities.into_iter().map(|m| m.into()).collect())
    }

    async fn list_by_province_code(
        &self,
        code: &str,
    ) -> Result<Vec<Municipality>, RepositoryError> {
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

    async fn list_by_district_code(
        &self,
        code: &str,
    ) -> Result<Vec<Municipality>, RepositoryError> {
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

    async fn list_barangays(&self, code: &str) -> Result<Vec<Barangay>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let barangays =
            models::barangay::Barangay::list_barangays_by_municipality_code(&mut executor, code)
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(barangays.into_iter().map(|b| b.into()).collect())
    }
}
