use actix_web::web::{self, Json, Query};
use psgc_application::{
    dto::{
        PaginateResponseDTO, city_dto::CityDTO, municipality_dto::MunicipalityDTO,
        province_dto::ProvinceDTO,
    },
    usecases::province_usecases::{
        GetProvinceByCodeUsecase, ListCitiesByProvinceUsecase, ListMunicipalitiesByProvinceUsecase,
        ListProvincesByRegionUsecase, ListProvincesUsecase,
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
        list_provinces,
        get_province_by_code,
        get_provinces_by_region,
        get_cities_by_province,
        get_municipalities_by_province,
    ),
    components(schemas(
        ProvinceDTO,
        PaginateResponseDTO<ProvinceDTO>,
        CityDTO,
        MunicipalityDTO,
        APIErr
    )),
    tags((
        name = "provinces", 
        description = "Philippine provinces management API"
    ))
)]
pub struct ProvinceAPIDoc;

pub fn build_province_route<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>() -> actix_web::Scope {
    web::scope("/provinces")
        .service(
            web::resource("")
                .route(web::get().to(list_provinces::<R, P, M, D, C, B>))
                .route(web::get().to(get_province_by_code::<R, P, M, D, C, B>)),
        )
        .service(
            web::resource("/region/{region_code}").route(web::get().to(get_provinces_by_region::<
                R,
                P,
                M,
                D,
                C,
                B,
            >)),
        )
        .service(
            web::resource("/{province_code}/cities")
                .route(web::get().to(get_cities_by_province::<R, P, M, D, C, B>)),
        )
        .service(
            web::resource("/{province_code}/municipalities")
                .route(web::get().to(get_municipalities_by_province::<R, P, M, D, C, B>)),
        )
}

#[utoipa::path(
    get,
    path = "/api/v1/provinces",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("limit" = Option<u32>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "Successfully retrieved provinces", body = PaginateResponseDTO<ProvinceDTO>),
        (status = 400, description = "Bad request", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "provinces",
    description = "Get all provinces"
)]
async fn list_provinces<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>(
    state: web::Data<APIState<R, P, M, D, C, B>>,
    Query(param): Query<PaginateQueryParam>,
) -> Result<Json<APIOk<PaginateResponseDTO<ProvinceDTO>>>, APIErr> {
    let province_repository = state.province_repository.clone();
    let list_provinces_usecase = ListProvincesUsecase::new(province_repository);

    let provinces = list_provinces_usecase
        .execute(param.page(), param.limit())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "All Provinces".to_string(),
        provinces,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/provinces/{code}",
    params(
        ("code" = String, Path, description = "Province code")
    ),
    responses(
        (status = 200, description = "Successfully retrieved province", body = ProvinceDTO),
        (status = 404, description = "Province not found", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "provinces",
    description = "Get province by code"
)]
async fn get_province_by_code<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>(
    state: web::Data<APIState<R, P, M, D, C, B>>,
    path: web::Path<String>,
) -> Result<Json<APIOk<ProvinceDTO>>, APIErr> {
    let province_repository = state.province_repository.clone();
    let get_province_by_code_usecase = GetProvinceByCodeUsecase::new(province_repository);

    let province = get_province_by_code_usecase
        .execute(&path.into_inner())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "Province details".to_string(),
        province,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/provinces/region/{region_code}",
    params(
        ("region_code" = String, Path, description = "Region code")
    ),
    responses(
        (status = 200, description = "Successfully retrieved provinces", body = Vec<ProvinceDTO>),
        (status = 400, description = "Bad request", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "provinces",
    description = "Get provinces by region"
)]
async fn get_provinces_by_region<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>(
    state: web::Data<APIState<R, P, M, D, C, B>>,
    path: web::Path<String>,
) -> Result<Json<APIOk<Vec<ProvinceDTO>>>, APIErr> {
    let province_repository = state.province_repository.clone();
    let list_provinces_by_region_usecase = ListProvincesByRegionUsecase::new(province_repository);

    let provinces = list_provinces_by_region_usecase
        .execute(&path.into_inner())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "Provinces by region".to_string(),
        provinces,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/provinces/{province_code}/cities",
    params(
        ("province_code" = String, Path, description = "Province code")
    ),
    responses(
        (status = 200, description = "Successfully retrieved cities", body = Vec<CityDTO>),
        (status = 400, description = "Bad request", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "provinces",
    description = "Get cities by province"
)]
async fn get_cities_by_province<
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
    let province_repository = state.province_repository.clone();
    let list_cities_by_province_usecase = ListCitiesByProvinceUsecase::new(province_repository);

    let cities = list_cities_by_province_usecase
        .execute(&path.into_inner())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "Cities by province".to_string(),
        cities,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/provinces/{province_code}/municipalities",
    params(
        ("province_code" = String, Path, description = "Province code")
    ),
    responses(
        (status = 200, description = "Successfully retrieved municipalities", body = Vec<MunicipalityDTO>),
        (status = 400, description = "Bad request", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "provinces",
    description = "Get municipalities by province"
)]
async fn get_municipalities_by_province<
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
    let province_repository = state.province_repository.clone();
    let list_municipalities_by_province_usecase =
        ListMunicipalitiesByProvinceUsecase::new(province_repository);

    let municipalities = list_municipalities_by_province_usecase
        .execute(&path.into_inner())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "Municipalities by province".to_string(),
        municipalities,
    )))
}
