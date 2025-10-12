use std::sync::Arc;

use psgc_domain::repositories::province_repository::ProvinceRepository;

use crate::{
    dto::{
        PaginateResponseDTO, city_dto::CityDTO, municipality_dto::MunicipalityDTO,
        province_dto::ProvinceDTO,
    },
    errors::UsecaseError,
};

#[derive(Debug)]
pub struct GetProvinceByCodeUsecase<P: ProvinceRepository> {
    province_repository: Arc<P>,
}

impl<P: ProvinceRepository> GetProvinceByCodeUsecase<P> {
    pub fn new(province_repository: Arc<P>) -> Self {
        Self {
            province_repository,
        }
    }

    fn province_repository(&self) -> &P {
        self.province_repository.as_ref()
    }

    pub async fn execute(&self, code: &str) -> Result<ProvinceDTO, UsecaseError> {
        let province_repository = self.province_repository();
        let province = province_repository.find_by_code(code).await?;

        Ok(province.into())
    }
}

#[derive(Debug)]
pub struct ListProvincesUsecase<P: ProvinceRepository> {
    province_repository: Arc<P>,
}

impl<P: ProvinceRepository> ListProvincesUsecase<P> {
    pub fn new(province_repository: Arc<P>) -> Self {
        Self {
            province_repository,
        }
    }

    fn province_repository(&self) -> &P {
        self.province_repository.as_ref()
    }

    pub async fn execute(
        &self,
        page: u64,
        limit: u64,
    ) -> Result<PaginateResponseDTO<ProvinceDTO>, UsecaseError> {
        let province_repository = self.province_repository();
        let provinces = province_repository.list_all(page, limit).await?;

        Ok(provinces.into())
    }
}

#[derive(Debug)]
pub struct ListProvincesByRegionUsecase<P: ProvinceRepository> {
    province_repository: Arc<P>,
}

impl<P: ProvinceRepository> ListProvincesByRegionUsecase<P> {
    pub fn new(province_repository: Arc<P>) -> Self {
        Self {
            province_repository,
        }
    }

    fn province_repository(&self) -> &P {
        self.province_repository.as_ref()
    }

    pub async fn execute(&self, region_code: &str) -> Result<Vec<ProvinceDTO>, UsecaseError> {
        let province_repository = self.province_repository();
        let provinces = province_repository.list_by_region_code(region_code).await?;
        let province_dtos = provinces.into_iter().map(|p| p.into()).collect();

        Ok(province_dtos)
    }
}

#[derive(Debug)]
pub struct ListCitiesByProvinceUsecase<P: ProvinceRepository> {
    province_repository: Arc<P>,
}

impl<P: ProvinceRepository> ListCitiesByProvinceUsecase<P> {
    pub fn new(province_repository: Arc<P>) -> Self {
        Self {
            province_repository,
        }
    }

    fn province_repository(&self) -> &P {
        self.province_repository.as_ref()
    }

    pub async fn execute(&self, province_code: &str) -> Result<Vec<CityDTO>, UsecaseError> {
        let province_repository = self.province_repository();
        let cities = province_repository.list_cities(province_code).await?;
        let city_dtos = cities.into_iter().map(|c| c.into()).collect();

        Ok(city_dtos)
    }
}

#[derive(Debug)]
pub struct ListMunicipalitiesByProvinceUsecase<P: ProvinceRepository> {
    province_repository: Arc<P>,
}

impl<P: ProvinceRepository> ListMunicipalitiesByProvinceUsecase<P> {
    pub fn new(province_repository: Arc<P>) -> Self {
        Self {
            province_repository,
        }
    }

    fn province_repository(&self) -> &P {
        self.province_repository.as_ref()
    }

    pub async fn execute(&self, province_code: &str) -> Result<Vec<MunicipalityDTO>, UsecaseError> {
        let province_repository = self.province_repository();
        let municipalities = province_repository
            .list_municipalities(province_code)
            .await?;
        let municipality_dtos = municipalities.into_iter().map(|m| m.into()).collect();

        Ok(municipality_dtos)
    }
}
