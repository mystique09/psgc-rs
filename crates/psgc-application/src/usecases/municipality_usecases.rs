use std::sync::Arc;

use psgc_domain::repositories::municipality_repository::MunicipalityRepository;

use crate::{
    dto::{PaginateResponseDTO, barangay_dto::BarangayDTO, municipality_dto::MunicipalityDTO},
    errors::UsecaseError,
};

#[derive(Debug, bon::Builder)]
pub struct GetMunicipalityByCodeUsecase<M: MunicipalityRepository> {
    municipality_repository: Arc<M>,
}

impl<M: MunicipalityRepository> GetMunicipalityByCodeUsecase<M> {
    pub fn new(municipality_repository: Arc<M>) -> Self {
        Self {
            municipality_repository,
        }
    }

    fn municipality_repository(&self) -> &M {
        self.municipality_repository.as_ref()
    }

    pub async fn execute(&self, code: &str) -> Result<MunicipalityDTO, UsecaseError> {
        let municipality_repository = self.municipality_repository();
        let municipality = municipality_repository.find_by_code(code).await?;

        Ok(municipality.into())
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListMunicipalitiesUsecase<M: MunicipalityRepository> {
    municipality_repository: Arc<M>,
}

impl<M: MunicipalityRepository> ListMunicipalitiesUsecase<M> {
    pub fn new(municipality_repository: Arc<M>) -> Self {
        Self {
            municipality_repository,
        }
    }

    fn municipality_repository(&self) -> &M {
        self.municipality_repository.as_ref()
    }

    pub async fn execute(
        &self,
        page: u64,
        limit: u64,
    ) -> Result<PaginateResponseDTO<MunicipalityDTO>, UsecaseError> {
        let municipality_repository = self.municipality_repository();
        let municipalities = municipality_repository.list_all(page, limit).await?;

        Ok(municipalities.into())
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListMunicipalitiesByRegionUsecase<M: MunicipalityRepository> {
    municipality_repository: Arc<M>,
}

impl<M: MunicipalityRepository> ListMunicipalitiesByRegionUsecase<M> {
    pub fn new(municipality_repository: Arc<M>) -> Self {
        Self {
            municipality_repository,
        }
    }

    fn municipality_repository(&self) -> &M {
        self.municipality_repository.as_ref()
    }

    pub async fn execute(&self, region_code: &str) -> Result<Vec<MunicipalityDTO>, UsecaseError> {
        let municipality_repository = self.municipality_repository();
        let municipalities = municipality_repository
            .list_by_region_code(region_code)
            .await?;
        let municipality_dtos = municipalities.into_iter().map(|m| m.into()).collect();

        Ok(municipality_dtos)
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListMunicipalitiesByProvinceUsecase<M: MunicipalityRepository> {
    municipality_repository: Arc<M>,
}

impl<M: MunicipalityRepository> ListMunicipalitiesByProvinceUsecase<M> {
    pub fn new(municipality_repository: Arc<M>) -> Self {
        Self {
            municipality_repository,
        }
    }

    fn municipality_repository(&self) -> &M {
        self.municipality_repository.as_ref()
    }

    pub async fn execute(&self, province_code: &str) -> Result<Vec<MunicipalityDTO>, UsecaseError> {
        let municipality_repository = self.municipality_repository();
        let municipalities = municipality_repository
            .list_by_province_code(province_code)
            .await?;
        let municipality_dtos = municipalities.into_iter().map(|m| m.into()).collect();

        Ok(municipality_dtos)
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListMunicipalitiesByDistrictUsecase<M: MunicipalityRepository> {
    municipality_repository: Arc<M>,
}

impl<M: MunicipalityRepository> ListMunicipalitiesByDistrictUsecase<M> {
    pub fn new(municipality_repository: Arc<M>) -> Self {
        Self {
            municipality_repository,
        }
    }

    fn municipality_repository(&self) -> &M {
        self.municipality_repository.as_ref()
    }

    pub async fn execute(&self, district_code: &str) -> Result<Vec<MunicipalityDTO>, UsecaseError> {
        let municipality_repository = self.municipality_repository();
        let municipalities = municipality_repository
            .list_by_district_code(district_code)
            .await?;
        let municipality_dtos = municipalities.into_iter().map(|m| m.into()).collect();

        Ok(municipality_dtos)
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListBarangaysByMunicipalityUsecase<M: MunicipalityRepository> {
    municipality_repository: Arc<M>,
}

impl<M: MunicipalityRepository> ListBarangaysByMunicipalityUsecase<M> {
    pub fn new(municipality_repository: Arc<M>) -> Self {
        Self {
            municipality_repository,
        }
    }

    fn municipality_repository(&self) -> &M {
        self.municipality_repository.as_ref()
    }

    pub async fn execute(&self, municipality_code: &str) -> Result<Vec<BarangayDTO>, UsecaseError> {
        let municipality_repository = self.municipality_repository();
        let barangays = municipality_repository
            .list_barangays(municipality_code)
            .await?;
        let barangay_dtos = barangays.into_iter().map(|b| b.into()).collect();

        Ok(barangay_dtos)
    }
}
