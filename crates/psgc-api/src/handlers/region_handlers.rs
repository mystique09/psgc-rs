use actix_web::web::{self, Json, Query, get};
use psgc_application::{
    dto::{PaginateResponseDTO, region_dto::RegionDTO},
    usecases::region_usecases::ListRegionsUsecase,
};
use psgc_domain::repositories::{
    barangay_repository::BarangayRepository, city_repository::CityRepository,
    district_repository::DistrictRepository, municipality_repository::MunicipalityRepository,
    province_repository::ProvinceRepository, region_repository::RegionRepository,
};

use crate::{
    dto::PaginateQueryParam,
    response::{APIErr, APIOk},
    router::APIState,
};

#[derive(Debug, utoipa::OpenApi)]
pub struct RegionAPIDoc;

pub fn build_region_route<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>() -> actix_web::Scope {
    actix_web::web::scope("/regions").service(web::resource("").route(get().to(list_regions::<
        R,
        P,
        M,
        D,
        C,
        B,
    >)))
}

async fn list_regions<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>(
    state: actix_web::web::Data<APIState<R, P, M, D, C, B>>,
    Query(param): Query<PaginateQueryParam>,
) -> Result<Json<APIOk<PaginateResponseDTO<RegionDTO>>>, APIErr> {
    let region_repository = state.region_repository.clone();
    let list_regions_usecase = ListRegionsUsecase::new(region_repository);

    let regions = list_regions_usecase
        .execute(param.page(), param.limit())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "All Regions".to_string(),
        regions,
    )))
}
