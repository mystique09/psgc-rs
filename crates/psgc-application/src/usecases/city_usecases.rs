use std::sync::Arc;

use psgc_domain::repositories::city_repository::CityRepository;

use crate::{
    dto::{PaginateResponseDTO, barangay_dto::BarangayDTO, city_dto::CityDTO},
    errors::UsecaseError,
};

#[derive(Debug, bon::Builder)]
pub struct GetCityByCodeUsecase<C: CityRepository> {
    city_repository: Arc<C>,
}

impl<C: CityRepository> GetCityByCodeUsecase<C> {
    pub fn new(city_repository: Arc<C>) -> Self {
        Self { city_repository }
    }

    fn city_repository(&self) -> &C {
        self.city_repository.as_ref()
    }

    pub async fn execute(&self, code: &str) -> Result<CityDTO, UsecaseError> {
        let city_repository = self.city_repository();
        let city = city_repository.find_by_code(code).await?;

        Ok(city.into())
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListCitiesUsecase<C: CityRepository> {
    city_repository: Arc<C>,
}

impl<C: CityRepository> ListCitiesUsecase<C> {
    pub fn new(city_repository: Arc<C>) -> Self {
        Self { city_repository }
    }

    fn city_repository(&self) -> &C {
        self.city_repository.as_ref()
    }

    pub async fn execute(
        &self,
        page: u64,
        limit: u64,
    ) -> Result<PaginateResponseDTO<CityDTO>, UsecaseError> {
        let city_repository = self.city_repository();
        let cities = city_repository.list_all(page, limit).await?;

        Ok(cities.into())
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListCitiesByRegionUsecase<C: CityRepository> {
    city_repository: Arc<C>,
}

impl<C: CityRepository> ListCitiesByRegionUsecase<C> {
    pub fn new(city_repository: Arc<C>) -> Self {
        Self { city_repository }
    }

    fn city_repository(&self) -> &C {
        self.city_repository.as_ref()
    }

    pub async fn execute(&self, region_code: &str) -> Result<Vec<CityDTO>, UsecaseError> {
        let city_repository = self.city_repository();
        let cities = city_repository.list_by_region_code(region_code).await?;
        let city_dtos = cities.into_iter().map(|c| c.into()).collect();

        Ok(city_dtos)
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListCitiesByProvinceUsecase<C: CityRepository> {
    city_repository: Arc<C>,
}

impl<C: CityRepository> ListCitiesByProvinceUsecase<C> {
    pub fn new(city_repository: Arc<C>) -> Self {
        Self { city_repository }
    }

    fn city_repository(&self) -> &C {
        self.city_repository.as_ref()
    }

    pub async fn execute(&self, province_code: &str) -> Result<Vec<CityDTO>, UsecaseError> {
        let city_repository = self.city_repository();
        let cities = city_repository.list_by_province_code(province_code).await?;
        let city_dtos = cities.into_iter().map(|c| c.into()).collect();

        Ok(city_dtos)
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListBarangaysByCityUsecase<C: CityRepository> {
    city_repository: Arc<C>,
}

impl<C: CityRepository> ListBarangaysByCityUsecase<C> {
    pub fn new(city_repository: Arc<C>) -> Self {
        Self { city_repository }
    }

    fn city_repository(&self) -> &C {
        self.city_repository.as_ref()
    }

    pub async fn execute(&self, city_code: &str) -> Result<Vec<BarangayDTO>, UsecaseError> {
        let city_repository = self.city_repository();
        let barangays = city_repository.list_barangays(city_code).await?;
        let barangay_dtos = barangays.into_iter().map(|b| b.into()).collect();

        Ok(barangay_dtos)
    }
}
