use std::sync::Arc;

use psgc_domain::{
    errors::RepositoryError,
    models::{PaginateResult, barangay::Barangay},
    repositories::barangay_repository::BarangayRepository,
};
use rbatis::{PageRequest, RBatis};
use tracing::info;

use crate::database::{generators::PageExt, models};

pub struct PgBarangayRepository {
    db: Arc<RBatis>,
}

impl PgBarangayRepository {
    pub fn new(db: Arc<RBatis>) -> Self {
        Self { db }
    }
}

#[allow(unused)]
impl BarangayRepository for PgBarangayRepository {
    async fn find_by_code(&self, code: &str) -> Result<Barangay, RepositoryError> {
        let barangay = models::barangay::Barangay::select_by_code(self.db.as_ref(), code)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?
            .ok_or(RepositoryError::NotFound)?;

        Ok(barangay.into())
    }

    async fn list_all(
        &self,
        page: u64,
        limit: u64,
    ) -> Result<PaginateResult<Barangay>, RepositoryError> {
        let now = std::time::Instant::now();

        let barangays = models::barangay::Barangay::list_barangays(
            self.db.as_ref(),
            &PageRequest::new(page, limit),
        )
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        let finish = std::time::Instant::now();
        let dur = finish - now;

        info!("[REPO] Duration: {:?}", dur);

        Ok(barangays.into_domain::<Barangay>())
    }

    async fn list_by_city_code(&self, code: &str) -> Result<Vec<Barangay>, RepositoryError> {
        let barangays =
            models::barangay::Barangay::list_barangays_by_city_code(self.db.as_ref(), code)
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(barangays.into_iter().map(|b| b.into()).collect())
    }

    async fn list_by_municipality_code(
        &self,
        code: &str,
    ) -> Result<Vec<Barangay>, RepositoryError> {
        let barangays =
            models::barangay::Barangay::list_barangays_by_municipality_code(self.db.as_ref(), code)
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(barangays.into_iter().map(|b| b.into()).collect())
    }

    async fn list_by_district_code(&self, code: &str) -> Result<Vec<Barangay>, RepositoryError> {
        let barangays =
            models::barangay::Barangay::list_barangays_by_district_code(self.db.as_ref(), code)
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(barangays.into_iter().map(|b| b.into()).collect())
    }
}
