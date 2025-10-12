use std::sync::Arc;

use psgc_domain::repositories::region_repository::RegionRepository;

use crate::{
    dto::{
        PaginateResponseDTO, city_dto::CityDTO, municipality_dto::MunicipalityDTO,
        province_dto::ProvinceDTO, region_dto::RegionDTO,
    },
    errors::UsecaseError,
};

#[derive(Debug, bon::Builder)]
pub struct GetRegionByCodeUsecase<R: RegionRepository> {
    region_repository: Arc<R>,
}

impl<R: RegionRepository> GetRegionByCodeUsecase<R> {
    pub fn new(region_repository: Arc<R>) -> Self {
        Self { region_repository }
    }

    fn region_repository(&self) -> &R {
        self.region_repository.as_ref()
    }

    pub async fn execute(&self, codename: &str) -> Result<RegionDTO, UsecaseError> {
        let region_repository = self.region_repository();
        let region = region_repository.find_by_code(codename).await?;

        Ok(region.into())
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListRegionsUsecase<R: RegionRepository> {
    region_repository: Arc<R>,
}

impl<R: RegionRepository> ListRegionsUsecase<R> {
    pub fn new(region_repository: Arc<R>) -> Self {
        Self { region_repository }
    }

    fn region_repository(&self) -> &R {
        self.region_repository.as_ref()
    }

    pub async fn execute(
        &self,
        region_id: &uuid::Uuid,
        page: u64,
        limit: u64,
    ) -> Result<PaginateResponseDTO<RegionDTO>, UsecaseError> {
        let region_repository = self.region_repository();
        let regions = region_repository.list_all(region_id, page, limit).await?;

        Ok(regions.into())
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListProvincesByRegionUsecase<R: RegionRepository> {
    region_repository: Arc<R>,
}

impl<R: RegionRepository> ListProvincesByRegionUsecase<R> {
    pub fn new(region_repository: Arc<R>) -> Self {
        Self { region_repository }
    }

    fn region_repository(&self) -> &R {
        self.region_repository.as_ref()
    }

    pub async fn execute(&self, code: &str) -> Result<Vec<ProvinceDTO>, UsecaseError> {
        let region_repository = self.region_repository();
        let provinces = region_repository.list_provinces(code).await?;
        let province_dtos = provinces.into_iter().map(|p| p.into()).collect();

        Ok(province_dtos)
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListCitiesByRegionUsecase<R: RegionRepository> {
    region_repository: Arc<R>,
}

impl<R: RegionRepository> ListCitiesByRegionUsecase<R> {
    pub fn new(region_repository: Arc<R>) -> Self {
        Self { region_repository }
    }

    fn region_repository(&self) -> &R {
        self.region_repository.as_ref()
    }

    pub async fn execute(&self, code: &str) -> Result<Vec<CityDTO>, UsecaseError> {
        let region_repository = self.region_repository();
        let cities = region_repository.list_cities(code).await?;
        let city_dtos = cities.into_iter().map(|c| c.into()).collect();

        Ok(city_dtos)
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListMunicipalitiesByRegionUsecase<R: RegionRepository> {
    region_repository: Arc<R>,
}

impl<R: RegionRepository> ListMunicipalitiesByRegionUsecase<R> {
    pub fn new(region_repository: Arc<R>) -> Self {
        Self { region_repository }
    }

    fn region_repository(&self) -> &R {
        self.region_repository.as_ref()
    }

    pub async fn execute(&self, code: &str) -> Result<Vec<MunicipalityDTO>, UsecaseError> {
        let region_repository = self.region_repository();
        let municipalities = region_repository.list_municipalities(code).await?;
        let municipality_dtos = municipalities.into_iter().map(|m| m.into()).collect();

        Ok(municipality_dtos)
    }
}
