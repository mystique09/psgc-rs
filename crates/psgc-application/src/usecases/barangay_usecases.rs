use std::sync::Arc;

use psgc_domain::repositories::barangay_repository::BarangayRepository;

use crate::{
    dto::{PaginateResponseDTO, barangay_dto::BarangayDTO},
    errors::UsecaseError,
};

#[derive(Debug, bon::Builder)]
pub struct GetBarangayByCodeUsecase<B: BarangayRepository> {
    barangay_repository: Arc<B>,
}

impl<B: BarangayRepository> GetBarangayByCodeUsecase<B> {
    pub fn new(barangay_repository: Arc<B>) -> Self {
        Self {
            barangay_repository,
        }
    }

    fn barangay_repository(&self) -> &B {
        self.barangay_repository.as_ref()
    }

    pub async fn execute(&self, code: &str) -> Result<BarangayDTO, UsecaseError> {
        let barangay_repository = self.barangay_repository();
        let barangay = barangay_repository.find_by_code(code).await?;

        Ok(barangay.into())
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListBarangaysUsecase<B: BarangayRepository> {
    barangay_repository: Arc<B>,
}

impl<B: BarangayRepository> ListBarangaysUsecase<B> {
    pub fn new(barangay_repository: Arc<B>) -> Self {
        Self {
            barangay_repository,
        }
    }

    fn barangay_repository(&self) -> &B {
        self.barangay_repository.as_ref()
    }

    pub async fn execute(
        &self,
        page: u64,
        limit: u64,
    ) -> Result<PaginateResponseDTO<BarangayDTO>, UsecaseError> {
        let barangay_repository = self.barangay_repository();
        let barangays = barangay_repository.list_all(page, limit).await?;

        Ok(barangays.into())
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListBarangaysByCityUsecase<B: BarangayRepository> {
    barangay_repository: Arc<B>,
}

impl<B: BarangayRepository> ListBarangaysByCityUsecase<B> {
    pub fn new(barangay_repository: Arc<B>) -> Self {
        Self {
            barangay_repository,
        }
    }

    fn barangay_repository(&self) -> &B {
        self.barangay_repository.as_ref()
    }

    pub async fn execute(&self, city_code: &str) -> Result<Vec<BarangayDTO>, UsecaseError> {
        let barangay_repository = self.barangay_repository();
        let barangays = barangay_repository.list_by_city_code(city_code).await?;
        let barangay_dtos = barangays.into_iter().map(|b| b.into()).collect();

        Ok(barangay_dtos)
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListBarangaysByMunicipalityUsecase<B: BarangayRepository> {
    barangay_repository: Arc<B>,
}

impl<B: BarangayRepository> ListBarangaysByMunicipalityUsecase<B> {
    pub fn new(barangay_repository: Arc<B>) -> Self {
        Self {
            barangay_repository,
        }
    }

    fn barangay_repository(&self) -> &B {
        self.barangay_repository.as_ref()
    }

    pub async fn execute(&self, municipality_code: &str) -> Result<Vec<BarangayDTO>, UsecaseError> {
        let barangay_repository = self.barangay_repository();
        let barangays = barangay_repository
            .list_by_municipality_code(municipality_code)
            .await?;
        let barangay_dtos = barangays.into_iter().map(|b| b.into()).collect();

        Ok(barangay_dtos)
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListBarangaysByDistrictUsecase<B: BarangayRepository> {
    barangay_repository: Arc<B>,
}

impl<B: BarangayRepository> ListBarangaysByDistrictUsecase<B> {
    pub fn new(barangay_repository: Arc<B>) -> Self {
        Self {
            barangay_repository,
        }
    }

    fn barangay_repository(&self) -> &B {
        self.barangay_repository.as_ref()
    }

    pub async fn execute(&self, district_code: &str) -> Result<Vec<BarangayDTO>, UsecaseError> {
        let barangay_repository = self.barangay_repository();
        let barangays = barangay_repository
            .list_by_district_code(district_code)
            .await?;
        let barangay_dtos = barangays.into_iter().map(|b| b.into()).collect();

        Ok(barangay_dtos)
    }
}
