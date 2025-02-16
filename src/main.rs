mod authentication;
mod configuration;
mod enc;
mod physical;
mod routers;

use crate::{authentication::Authentication, physical::Physical, routers::axum_server};
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

#[derive(Clone, Debug)]
struct AppState {
    physical: Physical,
    authentication: Authentication,
}

// #[tokio::main]
fn main() {
    tracing_subscriber::fmt()
        .with_thread_names(false)
        .with_line_number(true)
        .with_max_level(tracing::Level::INFO)
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("Starting Application...");
    let configuration = configuration::load_configuration();
    debug!("Server configuration: {:?}", configuration);
    let server = configuration.server.clone();
    let app_state = AppState {
        physical: Physical::new(configuration.physical.clone()),
        authentication: Authentication::new(configuration.authentication.clone()),
    };

    let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap();
    rt.block_on(async {
        axum_server(server, app_state).await;
    });
}
