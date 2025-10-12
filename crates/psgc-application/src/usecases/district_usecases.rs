use std::sync::Arc;

use psgc_domain::repositories::district_repository::DistrictRepository;

use crate::{
    dto::{
        PaginateResponseDTO, city_dto::CityDTO, district_dto::DistrictDTO,
        municipality_dto::MunicipalityDTO,
    },
    errors::UsecaseError,
};

#[derive(Debug, bon::Builder)]
pub struct GetDistrictByCodeUsecase<D: DistrictRepository> {
    district_repository: Arc<D>,
}

impl<D: DistrictRepository> GetDistrictByCodeUsecase<D> {
    pub fn new(district_repository: Arc<D>) -> Self {
        Self {
            district_repository,
        }
    }

    fn district_repository(&self) -> &D {
        self.district_repository.as_ref()
    }

    pub async fn execute(&self, code: &str) -> Result<DistrictDTO, UsecaseError> {
        let district_repository = self.district_repository();
        let district = district_repository.find_by_code(code).await?;

        Ok(district.into())
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListDistrictsUsecase<D: DistrictRepository> {
    district_repository: Arc<D>,
}

impl<D: DistrictRepository> ListDistrictsUsecase<D> {
    pub fn new(district_repository: Arc<D>) -> Self {
        Self {
            district_repository,
        }
    }

    fn district_repository(&self) -> &D {
        self.district_repository.as_ref()
    }

    pub async fn execute(
        &self,
        page: u64,
        limit: u64,
    ) -> Result<PaginateResponseDTO<DistrictDTO>, UsecaseError> {
        let district_repository = self.district_repository();
        let districts = district_repository.list_all(page, limit).await?;

        Ok(districts.into())
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListDistrictsByRegionUsecase<D: DistrictRepository> {
    district_repository: Arc<D>,
}

impl<D: DistrictRepository> ListDistrictsByRegionUsecase<D> {
    pub fn new(district_repository: Arc<D>) -> Self {
        Self {
            district_repository,
        }
    }

    fn district_repository(&self) -> &D {
        self.district_repository.as_ref()
    }

    pub async fn execute(&self, region_code: &str) -> Result<Vec<DistrictDTO>, UsecaseError> {
        let district_repository = self.district_repository();
        let districts = district_repository.list_by_region_code(region_code).await?;
        let district_dtos = districts.into_iter().map(|d| d.into()).collect();

        Ok(district_dtos)
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListDistrictsByProvinceUsecase<D: DistrictRepository> {
    district_repository: Arc<D>,
}

impl<D: DistrictRepository> ListDistrictsByProvinceUsecase<D> {
    pub fn new(district_repository: Arc<D>) -> Self {
        Self {
            district_repository,
        }
    }

    fn district_repository(&self) -> &D {
        self.district_repository.as_ref()
    }

    pub async fn execute(&self, province_code: &str) -> Result<Vec<DistrictDTO>, UsecaseError> {
        let district_repository = self.district_repository();
        let districts = district_repository
            .list_by_province_code(province_code)
            .await?;
        let district_dtos = districts.into_iter().map(|d| d.into()).collect();

        Ok(district_dtos)
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListCitiesByDistrictUsecase<D: DistrictRepository> {
    district_repository: Arc<D>,
}

impl<D: DistrictRepository> ListCitiesByDistrictUsecase<D> {
    pub fn new(district_repository: Arc<D>) -> Self {
        Self {
            district_repository,
        }
    }

    fn district_repository(&self) -> &D {
        self.district_repository.as_ref()
    }

    pub async fn execute(&self, district_code: &str) -> Result<Vec<CityDTO>, UsecaseError> {
        let district_repository = self.district_repository();
        let cities = district_repository.list_cities(district_code).await?;
        let city_dtos = cities.into_iter().map(|c| c.into()).collect();

        Ok(city_dtos)
    }
}

#[derive(Debug, bon::Builder)]
pub struct ListMunicipalitiesByDistrictUsecase<D: DistrictRepository> {
    district_repository: Arc<D>,
}

impl<D: DistrictRepository> ListMunicipalitiesByDistrictUsecase<D> {
    pub fn new(district_repository: Arc<D>) -> Self {
        Self {
            district_repository,
        }
    }

    fn district_repository(&self) -> &D {
        self.district_repository.as_ref()
    }

    pub async fn execute(&self, district_code: &str) -> Result<Vec<MunicipalityDTO>, UsecaseError> {
        let district_repository = self.district_repository();
        let municipalities = district_repository
            .list_municipalities(district_code)
            .await?;
        let municipality_dtos = municipalities.into_iter().map(|m| m.into()).collect();

        Ok(municipality_dtos)
    }
}
