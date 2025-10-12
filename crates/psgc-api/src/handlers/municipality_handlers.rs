use actix_web::web::{self, Json, Query};
use psgc_application::{
    dto::{PaginateResponseDTO, barangay_dto::BarangayDTO, municipality_dto::MunicipalityDTO},
    usecases::municipality_usecases::{
        GetMunicipalityByCodeUsecase, ListBarangaysByMunicipalityUsecase,
        ListMunicipalitiesByDistrictUsecase, ListMunicipalitiesByProvinceUsecase,
        ListMunicipalitiesByRegionUsecase, ListMunicipalitiesUsecase,
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
        list_municipalities,
        get_municipality_by_code,
        get_municipalities_by_region,
        get_municipalities_by_province,
        get_municipalities_by_district,
        get_barangays_by_municipality,
    ),
    components(schemas(
        MunicipalityDTO,
        PaginateResponseDTO<MunicipalityDTO>,
        BarangayDTO,
        APIErr
    )),
    tags((
        name = "municipalities", 
        description = "Philippine municipalities management API"
    ))
)]
pub struct MunicipalityAPIDoc;

pub fn build_municipality_route<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>() -> actix_web::Scope {
    web::scope("/municipalities")
        .service(
            web::resource("")
                .route(web::get().to(list_municipalities::<R, P, M, D, C, B>))
                .route(web::get().to(get_municipality_by_code::<R, P, M, D, C, B>)),
        )
        .service(
            web::resource("/region/{region_code}")
                .route(web::get().to(get_municipalities_by_region::<R, P, M, D, C, B>)),
        )
        .service(
            web::resource("/province/{province_code}")
                .route(web::get().to(get_municipalities_by_province::<R, P, M, D, C, B>)),
        )
        .service(
            web::resource("/district/{district_code}")
                .route(web::get().to(get_municipalities_by_district::<R, P, M, D, C, B>)),
        )
        .service(
            web::resource("/{municipality_code}/barangays")
                .route(web::get().to(get_barangays_by_municipality::<R, P, M, D, C, B>)),
        )
}

#[utoipa::path(
    get,
    path = "/api/v1/municipalities",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("limit" = Option<u32>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "Successfully retrieved municipalities", body = PaginateResponseDTO<MunicipalityDTO>),
        (status = 400, description = "Bad request", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "municipalities",
    description = "Get all municipalities"
)]
async fn list_municipalities<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>(
    state: web::Data<APIState<R, P, M, D, C, B>>,
    Query(param): Query<PaginateQueryParam>,
) -> Result<Json<APIOk<PaginateResponseDTO<MunicipalityDTO>>>, APIErr> {
    let municipality_repository = state.municipality_repository.clone();
    let list_municipalities_usecase = ListMunicipalitiesUsecase::new(municipality_repository);

    let municipalities = list_municipalities_usecase
        .execute(param.page(), param.limit())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "All Municipalities".to_string(),
        municipalities,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/municipalities/{code}",
    params(
        ("code" = String, Path, description = "Municipality code")
    ),
    responses(
        (status = 200, description = "Successfully retrieved municipality", body = MunicipalityDTO),
        (status = 404, description = "Municipality not found", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "municipalities",
    description = "Get municipality by code"
)]
async fn get_municipality_by_code<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>(
    state: web::Data<APIState<R, P, M, D, C, B>>,
    path: web::Path<String>,
) -> Result<Json<APIOk<MunicipalityDTO>>, APIErr> {
    let municipality_repository = state.municipality_repository.clone();
    let get_municipality_by_code_usecase =
        GetMunicipalityByCodeUsecase::new(municipality_repository);

    let municipality = get_municipality_by_code_usecase
        .execute(&path.into_inner())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "Municipality details".to_string(),
        municipality,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/municipalities/region/{region_code}",
    params(
        ("region_code" = String, Path, description = "Region code")
    ),
    responses(
        (status = 200, description = "Successfully retrieved municipalities", body = Vec<MunicipalityDTO>),
        (status = 400, description = "Bad request", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "municipalities",
    description = "Get municipalities by region"
)]
async fn get_municipalities_by_region<
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
    let municipality_repository = state.municipality_repository.clone();
    let list_municipalities_by_region_usecase =
        ListMunicipalitiesByRegionUsecase::new(municipality_repository);

    let municipalities = list_municipalities_by_region_usecase
        .execute(&path.into_inner())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "Municipalities by region".to_string(),
        municipalities,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/municipalities/province/{province_code}",
    params(
        ("province_code" = String, Path, description = "Province code")
    ),
    responses(
        (status = 200, description = "Successfully retrieved municipalities", body = Vec<MunicipalityDTO>),
        (status = 400, description = "Bad request", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "municipalities",
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
    let municipality_repository = state.municipality_repository.clone();
    let list_municipalities_by_province_usecase =
        ListMunicipalitiesByProvinceUsecase::new(municipality_repository);

    let municipalities = list_municipalities_by_province_usecase
        .execute(&path.into_inner())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "Municipalities by province".to_string(),
        municipalities,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/municipalities/district/{district_code}",
    params(
        ("district_code" = String, Path, description = "District code")
    ),
    responses(
        (status = 200, description = "Successfully retrieved municipalities", body = Vec<MunicipalityDTO>),
        (status = 400, description = "Bad request", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "municipalities",
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
    let municipality_repository = state.municipality_repository.clone();
    let list_municipalities_by_district_usecase =
        ListMunicipalitiesByDistrictUsecase::new(municipality_repository);

    let municipalities = list_municipalities_by_district_usecase
        .execute(&path.into_inner())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "Municipalities by district".to_string(),
        municipalities,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/municipalities/{municipality_code}/barangays",
    params(
        ("municipality_code" = String, Path, description = "Municipality code")
    ),
    responses(
        (status = 200, description = "Successfully retrieved barangays", body = Vec<BarangayDTO>),
        (status = 400, description = "Bad request", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "municipalities",
    description = "Get barangays by municipality"
)]
async fn get_barangays_by_municipality<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>(
    state: web::Data<APIState<R, P, M, D, C, B>>,
    path: web::Path<String>,
) -> Result<Json<APIOk<Vec<BarangayDTO>>>, APIErr> {
    let municipality_repository = state.municipality_repository.clone();
    let list_barangays_by_municipality_usecase =
        ListBarangaysByMunicipalityUsecase::new(municipality_repository);

    let barangays = list_barangays_by_municipality_usecase
        .execute(&path.into_inner())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "Barangays by municipality".to_string(),
        barangays,
    )))
}
