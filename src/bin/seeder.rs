use psgc_infrastructure::{
    config::db_config::DatabaseConfig,
    database::{pool::create_db_pool, seeder::seeder},
};
use tracing::info;
use tracing_log::LogTracer;
use tracing_subscriber::{Layer, layer::SubscriberExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_tracing();

    let config = DatabaseConfig::from_env()?;
    let pool = create_db_pool(&config)?;

    info!("Initializing PSGC database seed...");
    seeder(pool).await?;
    info!("PSGC database seeding completed successfully.");

    Ok(())
}

pub fn setup_tracing() {
    let crate_name = env!("CARGO_CRATE_NAME");
    let crate_version = env!("CARGO_PKG_VERSION");

    let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        format!(
            "RUST_LOG=info,{}=info,psgc_infrastructure=info,tokio=trace,runtime=trace,rbatis=info",
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
