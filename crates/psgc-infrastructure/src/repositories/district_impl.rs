use std::sync::Arc;

use psgc_domain::{
    errors::RepositoryError,
    models::{
        PaginateResult, barangay::Barangay, city::City, district::District,
        municipality::Municipality,
    },
    repositories::district_repository::DistrictRepository,
};
use rbatis::{PageRequest, RBatis};

use crate::database::{
    generators::{PageExt, UuidExt},
    models,
};

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

    async fn list_by_region_id(
        &self,
        region_id: &uuid::Uuid,
    ) -> Result<Vec<District>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let db_region_id = region_id.into_db();
        let districts =
            models::district::District::list_districts_by_region_id(&mut executor, &db_region_id)
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(districts.into_iter().map(|d| d.into()).collect())
    }

    async fn list_by_province_id(
        &self,
        province_id: &uuid::Uuid,
    ) -> Result<Vec<District>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let db_province_id = province_id.into_db();
        let districts = models::district::District::list_districts_by_province_id(
            &mut executor,
            &db_province_id,
        )
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(districts.into_iter().map(|d| d.into()).collect())
    }

    async fn list_cities_by_district_id(
        &self,
        _district_id: &uuid::Uuid,
    ) -> Result<Vec<City>, RepositoryError> {
        // Cities don't have direct district_id relationship in current model structure
        // This would require more complex queries or schema changes
        // For now, return empty vector as this relationship isn't implemented
        Ok(Vec::new())
    }

    async fn list_municipalities_by_district_id(
        &self,
        district_id: &uuid::Uuid,
    ) -> Result<Vec<Municipality>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let db_district_id = district_id.into_db();
        let municipalities =
            models::municipality::Municipality::list_municipalities_by_district_id(
                &mut executor,
                &db_district_id,
            )
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(municipalities.into_iter().map(|m| m.into()).collect())
    }

    async fn list_barangays_by_district_id(
        &self,
        district_id: &uuid::Uuid,
    ) -> Result<Vec<Barangay>, RepositoryError> {
        let mut executor = self.db.acquire().await.unwrap();
        let db_district_id = district_id.into_db();

        // Get barangays from municipalities in this district
        let municipalities =
            models::municipality::Municipality::list_municipalities_by_district_id(
                &mut executor,
                &db_district_id,
            )
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut all_barangays = Vec::new();

        // Get barangays from municipalities in this district
        for municipality in municipalities {
            let municipality_barangays =
                models::barangay::Barangay::list_barangays_by_municipality_id(
                    &mut executor,
                    &municipality.id,
                )
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
            all_barangays.extend(municipality_barangays);
        }

        Ok(all_barangays.into_iter().map(|b| b.into()).collect())
    }
}
