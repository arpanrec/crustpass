mod auth;
mod enc;
mod physical;
mod routers;
mod settings;

use crate::{auth::Authentication, physical::Physical, routers::axum_server};
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Debug)]
struct AppState {
    physical: Physical,
    authentication: Authentication,
}

// #[tokio::main]
fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    info!("Starting Application...");
    let configuration = settings::load_configuration();
    debug!("Server configuration: {:?}", configuration);
    let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap();
    rt.block_on(async {
        axum_server(configuration).await;
    });
}
