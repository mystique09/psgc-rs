use actix_web::web::{self, Json, Query};
use psgc_application::{
    dto::{PaginateResponseDTO, barangay_dto::BarangayDTO},
    usecases::barangay_usecases::{
        GetBarangayByCodeUsecase, ListBarangaysByCityUsecase, ListBarangaysByDistrictUsecase,
        ListBarangaysByMunicipalityUsecase, ListBarangaysUsecase,
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
        list_barangays,
        get_barangay_by_code,
        get_barangays_by_city,
        get_barangays_by_municipality,
        get_barangays_by_district,
    ),
    components(schemas(
        BarangayDTO,
        PaginateResponseDTO<BarangayDTO>,
        APIErr
    )),
    tags((
        name = "barangays", 
        description = "Philippine barangays management API"
    ))
)]
pub struct BarangayAPIDoc;

pub fn build_barangay_route<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>() -> actix_web::Scope {
    web::scope("/barangays")
        .service(
            web::resource("")
                .route(web::get().to(list_barangays::<R, P, M, D, C, B>))
                .route(web::get().to(get_barangay_by_code::<R, P, M, D, C, B>)),
        )
        .service(
            web::resource("/city/{city_code}").route(web::get().to(get_barangays_by_city::<
                R,
                P,
                M,
                D,
                C,
                B,
            >)),
        )
        .service(
            web::resource("/municipality/{municipality_code}")
                .route(web::get().to(get_barangays_by_municipality::<R, P, M, D, C, B>)),
        )
        .service(
            web::resource("/district/{district_code}")
                .route(web::get().to(get_barangays_by_district::<R, P, M, D, C, B>)),
        )
}

#[utoipa::path(
    get,
    path = "/api/v1/barangays",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("limit" = Option<u32>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "Successfully retrieved barangays", body = PaginateResponseDTO<BarangayDTO>),
        (status = 400, description = "Bad request", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "barangays",
    description = "Get all barangays"
)]
async fn list_barangays<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>(
    state: web::Data<APIState<R, P, M, D, C, B>>,
    Query(param): Query<PaginateQueryParam>,
) -> Result<Json<APIOk<PaginateResponseDTO<BarangayDTO>>>, APIErr> {
    let barangay_repository = state.barangay_repository.clone();
    let list_barangays_usecase = ListBarangaysUsecase::new(barangay_repository);

    let barangays = list_barangays_usecase
        .execute(param.page(), param.limit())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "All Barangays".to_string(),
        barangays,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/barangays/{code}",
    params(
        ("code" = String, Path, description = "Barangay code")
    ),
    responses(
        (status = 200, description = "Successfully retrieved barangay", body = BarangayDTO),
        (status = 404, description = "Barangay not found", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "barangays",
    description = "Get barangay by code"
)]
async fn get_barangay_by_code<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>(
    state: web::Data<APIState<R, P, M, D, C, B>>,
    path: web::Path<String>,
) -> Result<Json<APIOk<BarangayDTO>>, APIErr> {
    let barangay_repository = state.barangay_repository.clone();
    let get_barangay_by_code_usecase = GetBarangayByCodeUsecase::new(barangay_repository);

    let barangay = get_barangay_by_code_usecase
        .execute(&path.into_inner())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "Barangay details".to_string(),
        barangay,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/barangays/city/{city_code}",
    params(
        ("city_code" = String, Path, description = "City code")
    ),
    responses(
        (status = 200, description = "Successfully retrieved barangays", body = Vec<BarangayDTO>),
        (status = 400, description = "Bad request", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "barangays",
    description = "Get barangays by city"
)]
async fn get_barangays_by_city<
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
    let barangay_repository = state.barangay_repository.clone();
    let list_barangays_by_city_usecase = ListBarangaysByCityUsecase::new(barangay_repository);

    let barangays = list_barangays_by_city_usecase
        .execute(&path.into_inner())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "Barangays by city".to_string(),
        barangays,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/barangays/municipality/{municipality_code}",
    params(
        ("municipality_code" = String, Path, description = "Municipality code")
    ),
    responses(
        (status = 200, description = "Successfully retrieved barangays", body = Vec<BarangayDTO>),
        (status = 400, description = "Bad request", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "barangays",
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
    let barangay_repository = state.barangay_repository.clone();
    let list_barangays_by_municipality_usecase =
        ListBarangaysByMunicipalityUsecase::new(barangay_repository);

    let barangays = list_barangays_by_municipality_usecase
        .execute(&path.into_inner())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "Barangays by municipality".to_string(),
        barangays,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/barangays/district/{district_code}",
    params(
        ("district_code" = String, Path, description = "District code")
    ),
    responses(
        (status = 200, description = "Successfully retrieved barangays", body = Vec<BarangayDTO>),
        (status = 400, description = "Bad request", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "barangays",
    description = "Get barangays by district"
)]
async fn get_barangays_by_district<
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
    let barangay_repository = state.barangay_repository.clone();
    let list_barangays_by_district_usecase =
        ListBarangaysByDistrictUsecase::new(barangay_repository);

    let barangays = list_barangays_by_district_usecase
        .execute(&path.into_inner())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "Barangays by district".to_string(),
        barangays,
    )))
}
