use actix_web::web::{self, Json, Query};
use psgc_application::{
    dto::{
        PaginateResponseDTO, city_dto::CityDTO, district_dto::DistrictDTO,
        municipality_dto::MunicipalityDTO,
    },
    usecases::district_usecases::{
        GetDistrictByCodeUsecase, ListCitiesByDistrictUsecase, ListDistrictsByProvinceUsecase,
        ListDistrictsByRegionUsecase, ListDistrictsUsecase, ListMunicipalitiesByDistrictUsecase,
    },
};
use psgc_domain::repositories::{
    barangay_repository::BarangayRepository, city_repository::CityRepository,
    district_repository::DistrictRepository, municipality_repository::MunicipalityRepository,
    province_repository::ProvinceRepository, region_repository::RegionRepository,
};
use utoipa::OpenApi;

use crate::{
    dto::PaginateQueryParam,
    response::{APIErr, APIOk},
    router::APIState,
};

#[derive(Debug, OpenApi)]
#[openapi(
    paths(
        list_districts,
        get_district_by_code,
        get_districts_by_region,
        get_districts_by_province,
        get_cities_by_district,
        get_municipalities_by_district,
    ),
    components(schemas(
        DistrictDTO,
        PaginateResponseDTO<DistrictDTO>,
        CityDTO,
        MunicipalityDTO,
        APIErr
    )),
    tags((
        name = "districts", 
        description = "Philippine districts management API"
    ))
)]
pub struct DistrictAPIDoc;

pub fn build_district_route<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>() -> actix_web::Scope {
    web::scope("/districts")
        .service(
            web::resource("")
                .route(web::get().to(list_districts::<R, P, M, D, C, B>))
                .route(web::get().to(get_district_by_code::<R, P, M, D, C, B>)),
        )
        .service(
            web::resource("/region/{region_code}").route(web::get().to(get_districts_by_region::<
                R,
                P,
                M,
                D,
                C,
                B,
            >)),
        )
        .service(
            web::resource("/province/{province_code}")
                .route(web::get().to(get_districts_by_province::<R, P, M, D, C, B>)),
        )
        .service(
            web::resource("/{district_code}/cities")
                .route(web::get().to(get_cities_by_district::<R, P, M, D, C, B>)),
        )
        .service(
            web::resource("/{district_code}/municipalities")
                .route(web::get().to(get_municipalities_by_district::<R, P, M, D, C, B>)),
        )
}

#[utoipa::path(
    get,
    path = "/api/v1/districts",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("limit" = Option<u32>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "Successfully retrieved districts", body = PaginateResponseDTO<DistrictDTO>),
        (status = 400, description = "Bad request", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "districts",
    description = "Get all districts"
)]
async fn list_districts<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>(
    state: web::Data<APIState<R, P, M, D, C, B>>,
    Query(param): Query<PaginateQueryParam>,
) -> Result<Json<APIOk<PaginateResponseDTO<DistrictDTO>>>, APIErr> {
    let district_repository = state.district_repository.clone();
    let list_districts_usecase = ListDistrictsUsecase::new(district_repository);

    let districts = list_districts_usecase
        .execute(param.page(), param.limit())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "All Districts".to_string(),
        districts,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/districts/{code}",
    params(
        ("code" = String, Path, description = "District code")
    ),
    responses(
        (status = 200, description = "Successfully retrieved district", body = DistrictDTO),
        (status = 404, description = "District not found", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "districts",
    description = "Get district by code"
)]
async fn get_district_by_code<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>(
    state: web::Data<APIState<R, P, M, D, C, B>>,
    path: web::Path<String>,
) -> Result<Json<APIOk<DistrictDTO>>, APIErr> {
    let district_repository = state.district_repository.clone();
    let get_district_by_code_usecase = GetDistrictByCodeUsecase::new(district_repository);

    let district = get_district_by_code_usecase
        .execute(&path.into_inner())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "District details".to_string(),
        district,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/districts/region/{region_code}",
    params(
        ("region_code" = String, Path, description = "Region code")
    ),
    responses(
        (status = 200, description = "Successfully retrieved districts", body = Vec<DistrictDTO>),
        (status = 400, description = "Bad request", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "districts",
    description = "Get districts by region"
)]
async fn get_districts_by_region<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>(
    state: web::Data<APIState<R, P, M, D, C, B>>,
    path: web::Path<String>,
) -> Result<Json<APIOk<Vec<DistrictDTO>>>, APIErr> {
    let district_repository = state.district_repository.clone();
    let list_districts_by_region_usecase = ListDistrictsByRegionUsecase::new(district_repository);

    let districts = list_districts_by_region_usecase
        .execute(&path.into_inner())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "Districts by region".to_string(),
        districts,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/districts/province/{province_code}",
    params(
        ("province_code" = String, Path, description = "Province code")
    ),
    responses(
        (status = 200, description = "Successfully retrieved districts", body = Vec<DistrictDTO>),
        (status = 400, description = "Bad request", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "districts",
    description = "Get districts by province"
)]
async fn get_districts_by_province<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>(
    state: web::Data<APIState<R, P, M, D, C, B>>,
    path: web::Path<String>,
) -> Result<Json<APIOk<Vec<DistrictDTO>>>, APIErr> {
    let district_repository = state.district_repository.clone();
    let list_districts_by_province_usecase =
        ListDistrictsByProvinceUsecase::new(district_repository);

    let districts = list_districts_by_province_usecase
        .execute(&path.into_inner())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "Districts by province".to_string(),
        districts,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/districts/{district_code}/cities",
    params(
        ("district_code" = String, Path, description = "District code")
    ),
    responses(
        (status = 200, description = "Successfully retrieved cities", body = Vec<CityDTO>),
        (status = 400, description = "Bad request", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "districts",
    description = "Get cities by district"
)]
async fn get_cities_by_district<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>(
    state: web::Data<APIState<R, P, M, D, C, B>>,
    path: web::Path<String>,
) -> Result<Json<APIOk<Vec<CityDTO>>>, APIErr> {
    let district_repository = state.district_repository.clone();
    let list_cities_by_district_usecase = ListCitiesByDistrictUsecase::new(district_repository);

    let cities = list_cities_by_district_usecase
        .execute(&path.into_inner())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "Cities by district".to_string(),
        cities,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/districts/{district_code}/municipalities",
    params(
        ("district_code" = String, Path, description = "District code")
    ),
    responses(
        (status = 200, description = "Successfully retrieved municipalities", body = Vec<MunicipalityDTO>),
        (status = 400, description = "Bad request", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "districts",
    description = "Get municipalities by district"
)]
async fn get_municipalities_by_district<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>(
    state: web::Data<APIState<R, P, M, D, C, B>>,
    path: web::Path<String>,
) -> Result<Json<APIOk<Vec<MunicipalityDTO>>>, APIErr> {
    let district_repository = state.district_repository.clone();
    let list_municipalities_by_district_usecase =
        ListMunicipalitiesByDistrictUsecase::new(district_repository);

    let municipalities = list_municipalities_by_district_usecase
        .execute(&path.into_inner())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "Municipalities by district".to_string(),
        municipalities,
    )))
}
