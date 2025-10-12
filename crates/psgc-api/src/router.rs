use actix_governor::{Governor, GovernorConfigBuilder};
use std::sync::Arc;
use utoipa::OpenApi as OpenApiT;

use crate::{
    handlers::region_handlers::RegionAPIDoc,
    response::{APIErr, APIOk},
};
use actix_cors::Cors;
use actix_web::{
    App, Error, HttpResponse, Result,
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    http::{Method, StatusCode},
    middleware::{ErrorHandlerResponse, ErrorHandlers, Logger, NormalizePath},
    web::{self, Data, Json, get},
};
use psgc_domain::repositories::{
    barangay_repository::BarangayRepository, city_repository::CityRepository,
    district_repository::DistrictRepository, municipality_repository::MunicipalityRepository,
    province_repository::ProvinceRepository, region_repository::RegionRepository,
};
use scalar_doc::favicon::FaviconMimeType;
use utoipa::openapi::OpenApi;
use utoipa_actix_web::AppExt;

#[derive(Debug, bon::Builder)]
pub struct APIState<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
> {
    pub allowed_origins: Vec<String>,
    pub region_repository: Arc<R>,
    pub province_repository: Arc<P>,
    pub municipality_repository: Arc<M>,
    pub district_repository: Arc<D>,
    pub city_repository: Arc<C>,
    pub barangay_repository: Arc<B>,
}

#[derive(Debug, OpenApiT)]
pub struct PSGCApiDoc;

pub fn create_api_router<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>(
    state: APIState<R, P, M, D, C, B>,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let allowed_origins = state.allowed_origins.clone();

    let governor_config = GovernorConfigBuilder::default()
        .seconds_per_request(10)
        .burst_size(1000)
        .finish()
        .unwrap();
    let governor = Governor::new(&governor_config);

    let cors = Cors::default()
        .allowed_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_any_header()
        .max_age(3600)
        .allowed_origin_fn(move |origin, _req| {
            let origin_val = origin.to_str().unwrap_or("");
            let is_allowed = allowed_origins.contains(&origin_val.to_string());

            is_allowed
        })
        .supports_credentials();

    let (mut app, docs) = App::new()
        .wrap(Logger::default())
        .wrap(NormalizePath::trim())
        .wrap(cors)
        .wrap(governor)
        .wrap(ErrorHandlers::new().handler(
            StatusCode::NOT_FOUND,
            error_404_error_handler::<_, R, P, M, D, C, B>,
        ))
        .app_data(Data::new(state))
        .service(web::resource("/favicon.svg").route(get().to(favicon)))
        .service(web::resource("/").route(get().to(index::<R, P, M, D, C, B>)))
        .service(web::resource("/docs").route(get().to(docs)))
        .service(web::scope("/api/v1").route("", get().to(openapi_json)))
        .into_utoipa_app()
        .split_for_parts();

    let mut docs = docs
        .merge_from(PSGCApiDoc::openapi())
        .merge_from(RegionAPIDoc::openapi());
    docs.info.title = "PSGC API Documentation".to_string();
    docs.info.description = Some("API documentation for the PSGC API".to_string());
    docs.info.version = env!("CARGO_PKG_VERSION").to_string();

    let docs = Arc::new(docs);
    app = app.app_data(Data::new(docs));

    app
}

async fn favicon() -> Result<HttpResponse, APIErr> {
    let svg = include_str!("../../../assets/favicon.svg");
    Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
}

async fn index<
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    B: BarangayRepository,
>(
    state: Data<APIState<R, P, M, D, C, B>>,
) -> Result<Json<APIOk<String>>, APIErr> {
    let origin = state
        .allowed_origins
        .first()
        .map(|origin| origin.as_str())
        .unwrap_or("http://localhost:3000");

    let message = format!(
        "This is the API for PSGC, please visit <a href='{origin}'>{origin}</a>. For documentation, visit <a href='/docs'>{origin}/docs</a>"
    );

    Ok(Json(APIOk::builder().message(message).build()))
}

fn error_404_error_handler<B, R, P, M, D, C, Br>(
    res: ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>>
where
    R: RegionRepository,
    P: ProvinceRepository,
    M: MunicipalityRepository,
    D: DistrictRepository,
    C: CityRepository,
    Br: BarangayRepository,
{
    let (req, _) = res.into_parts();
    let state = req.app_data::<Data<APIState<R, P, M, D, C, Br>>>().unwrap();

    let origin = state
        .allowed_origins
        .first()
        .map(|origin| origin.as_str())
        .unwrap_or("http://localhost:3000");

    let message = format!("Seems like you are lost, please visit <a href='{origin}'>{origin}</a>");

    let response = HttpResponse::NotFound()
        .content_type("application/json")
        .json(Json(APIOk::<String>::builder().message(message).build()));

    Ok(ErrorHandlerResponse::Response(
        ServiceResponse::new(req, response).map_into_right_body(),
    ))
}

async fn openapi_json(docs: Data<Arc<OpenApi>>) -> HttpResponse {
    HttpResponse::Ok().json(docs.as_ref())
}

async fn docs() -> HttpResponse {
    let scalar = scalar_doc::Documentation::new("Reforged API Documentation", "/api/v1")
        .favicon("/favicon.svg", FaviconMimeType::Svg)
        .build()
        .unwrap();

    HttpResponse::Ok().body(scalar)
}
