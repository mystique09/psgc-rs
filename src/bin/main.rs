use std::sync::Arc;

use actix_web::{HttpServer, dev::ServerHandle, rt::signal};
use psgc_api::router::{APIState, create_api_router};
use psgc_infrastructure::{
    config::{db_config::DatabaseConfig, http_config::HTTPConfig},
    database::pool::create_db_pool,
    repositories::{
        barangay_impl::PgBarangayRepository, city_impl::PgCityRepository,
        district_impl::PgDistrictRepository, municipality_impl::PgMunicipalityRepository,
        province_impl::PgProvinceRepository, region_impl::PgRegionRepository,
    },
};
use tracing::info;
use tracing_log::LogTracer;
use tracing_subscriber::{Layer, layer::SubscriberExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_tracing();

    let http_config = HTTPConfig::from_env()?;
    let db_config = DatabaseConfig::from_env()?;
    let db = create_db_pool(&db_config)?;

    let region_repository = Arc::new(PgRegionRepository::new(db.clone()));

    let province_repository = Arc::new(PgProvinceRepository::new(db.clone()));
    let municipality_repository = Arc::new(PgMunicipalityRepository::new(db.clone()));
    let district_repository = Arc::new(PgDistrictRepository::new(db.clone()));
    let city_repository = Arc::new(PgCityRepository::new(db.clone()));
    let barangay_repository = Arc::new(PgBarangayRepository::new(db));

    let http_addr = http_config.get_connection_string();
    let http_addr_clone = http_config.get_connection_string();

    tokio::spawn(async move {
        let http_addr = http_addr_clone.clone();
        info!("listening on {}", http_addr);
    });

    let server = HttpServer::new(move || {
        let api_state = APIState::builder()
            .allowed_origins(http_config.allowed_origins.clone())
            .region_repository(region_repository.clone())
            .province_repository(province_repository.clone())
            .municipality_repository(municipality_repository.clone())
            .district_repository(district_repository.clone())
            .city_repository(city_repository.clone())
            .barangay_repository(barangay_repository.clone())
            .build();

        let router = create_api_router(api_state);
        router
    })
    .workers(2)
    .bind(http_addr)?
    // .bind_rustls_0_23(http_addr, tls)?
    .shutdown_timeout(5)
    .run();

    let server_handle = server.handle();

    tokio::spawn(async move {
        shutdown_signal(server_handle).await;
    });

    server.await?;

    Ok(())
}

pub fn setup_tracing() {
    let crate_name = env!("CARGO_CRATE_NAME");
    let crate_version = env!("CARGO_PKG_VERSION");

    let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        format!(
            "RUST_LOG=info,{}=info,psgc_infrastructure=info,psgc_application=info,psgc_api=info,tokio=trace,runtime=trace,rbatis=info,actix_web=info",
            crate_name
        )
        .into()
    });

    let fmt_layer = tracing_subscriber::fmt::layer().with_filter(filter_layer);
    let subscriber = tracing_subscriber::registry().with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");

    LogTracer::init().expect("Failed to set logger");

    info!("[PSGC-RS] {} v{}", crate_name, crate_version);
}

pub async fn shutdown_signal(handle: ServerHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to initialize Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to initialize signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("gracefully shutting down server...");
    info!("closing database pool connections...");
    info!("successfully closed database pool connections");
    info!("shutting down server...");
    info!("server shutdown complete");
    info!("goodbye!");

    handle.stop(true).await;
}
