use actix_web::web::{self, Json, Query};
use psgc_application::{
    dto::{PaginateResponseDTO, barangay_dto::BarangayDTO, city_dto::CityDTO},
    usecases::city_usecases::{
        GetCityByCodeUsecase, ListBarangaysByCityUsecase, ListCitiesUsecase,
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
        list_cities,
        get_city_by_code,
        get_barangays_by_city,
    ),
    components(schemas(
        CityDTO,
        PaginateResponseDTO<CityDTO>,
        BarangayDTO,
        APIErr
    )),
    tags((
        name = "cities", 
        description = "Philippine cities management API"
    ))
)]
pub struct CityAPIDoc;

pub fn build_city_route<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>() -> actix_web::Scope {
    web::scope("/cities")
        .service(web::resource("").route(web::get().to(list_cities::<R, P, M, D, C, B>)))
        .service(
            web::resource("/{code}").route(web::get().to(get_city_by_code::<R, P, M, D, C, B>)),
        )
        .service(
            web::resource("/{city_code}/barangays").route(web::get().to(get_barangays_by_city::<
                R,
                P,
                M,
                D,
                C,
                B,
            >)),
        )
}

#[utoipa::path(
    get,
    path = "/api/v1/cities",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("limit" = Option<u32>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "Successfully retrieved cities", body = PaginateResponseDTO<CityDTO>),
        (status = 400, description = "Bad request", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "cities",
    description = "Get all cities"
)]
async fn list_cities<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>(
    state: web::Data<APIState<R, P, M, D, C, B>>,
    Query(param): Query<PaginateQueryParam>,
) -> Result<Json<APIOk<PaginateResponseDTO<CityDTO>>>, APIErr> {
    let city_repository = state.city_repository.clone();
    let list_cities_usecase = ListCitiesUsecase::new(city_repository);

    let cities = list_cities_usecase
        .execute(param.page(), param.limit())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "All Cities".to_string(),
        cities,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/cities/{code}",
    params(
        ("code" = String, Path, description = "City code")
    ),
    responses(
        (status = 200, description = "Successfully retrieved city", body = CityDTO),
        (status = 404, description = "City not found", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "cities",
    description = "Get city by code"
)]
async fn get_city_by_code<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>(
    state: web::Data<APIState<R, P, M, D, C, B>>,
    path: web::Path<String>,
) -> Result<Json<APIOk<CityDTO>>, APIErr> {
    let city_repository = state.city_repository.clone();
    let get_city_by_code_usecase = GetCityByCodeUsecase::new(city_repository);

    let city = get_city_by_code_usecase.execute(&path.into_inner()).await?;

    Ok(Json(APIOk::success_with_message(
        "City details".to_string(),
        city,
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/cities/{city_code}/barangays",
    params(
        ("city_code" = String, Path, description = "City code")
    ),
    responses(
        (status = 200, description = "Successfully retrieved barangays", body = Vec<BarangayDTO>),
        (status = 400, description = "Bad request", body = APIErr),
        (status = 500, description = "Internal server error", body = APIErr)
    ),
    tag = "cities",
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
    let city_repository = state.city_repository.clone();
    let list_barangays_by_city_usecase = ListBarangaysByCityUsecase::new(city_repository);

    let barangays = list_barangays_by_city_usecase
        .execute(&path.into_inner())
        .await?;

    Ok(Json(APIOk::success_with_message(
        "Barangays by city".to_string(),
        barangays,
    )))
}
