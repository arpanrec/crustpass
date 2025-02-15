mod app_settings;
mod physical;

use crate::app_settings::Configuration;
use crate::physical::Physical;
use axum::{
    extract,
    extract::Path,
    extract::{ConnectInfo, State},
    http::{Response, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    routing::post,
    Router,
};
use std::net::SocketAddr;
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Debug)]
struct AppState {
    storage: Physical,
    configuration: Configuration,
}

// #[tokio::main]
fn main() {
    println!("Starting Secret Squirrel...");
    let configuration = app_settings::load_configuration();
    let storage = Physical::new(configuration.physical.clone());
    let app_state = AppState { storage, configuration };
    println!("Server configuration: {:?}", app_state);
    let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap();
    rt.block_on(async {
        axum_server(app_state).await;
    });
}

async fn axum_server(app_state: AppState) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let addr: SocketAddr = app_state.configuration.socket_addr.parse().unwrap();
    let app = Router::new()
        .route("/secret/{*key}", get(handle_get).delete(handle_delete))
        .route("/secret/{*key}", post(handle_post))
        .route("/{*key}", axum::routing::any(handle_any))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_layer))
        .with_state(app_state);
    info!("Starting server on: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

async fn handle_get(Path(path): Path<String>, State(state): State<AppState>) -> Response<String> {
    info!("Received get request for key: {}", path);
    let mut storage = state.storage;
    let value = storage.read(&path).await;
    debug!("Read value: {:?}", value);
    let builder = Response::builder();
    if let Some(value) = value {
        debug!("Found value: {}", value);
        builder
            .status(StatusCode::OK)
            .header("Content-Type", "text/plain")
            .body(value)
            .expect("Failed to send response")
    } else {
        debug!("Not Found");
        builder
            .status(StatusCode::NOT_FOUND)
            .header("Content-Type", "text/plain")
            .body("".to_string())
            .expect("Failed to send response")
    }
}

async fn handle_post(Path(path): Path<String>, State(state): State<AppState>, body: String) -> Response<String> {
    info!("Received post request for key: {}", path);
    debug!("Received body: {}", body);
    let mut storage = state.storage;
    storage.write(&path, &body).await;
    Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "text/plain")
        .body("".to_string())
        .expect("Failed to send response")
}

async fn handle_delete(Path(path): Path<String>, State(state): State<AppState>) -> Response<String> {
    info!("Received delete request for key: {}", path);
    let mut storage = state.storage;
    storage.delete(&path).await;
    Response::new("".to_string())
}

async fn handle_any() -> Response<String> {
    Response::new("".to_string())
}

async fn auth_layer(
    State(app_state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: extract::Request,
    next: Next,
) -> impl IntoResponse {
    let admin_api_key =
        app_state.configuration.authentication.authentication_details["admin_api_key"].as_str().unwrap();
    let mut is_authorized = false;
    if let Some(header) = request.headers().get("Authorization") {
        if let Ok(value) = header.to_str() {
            if value == admin_api_key {
                is_authorized = true;
            }
        }
    }
    if is_authorized {
        next.run(request).await.into_response()
    } else {
        info!("Unauthorized request from: {:?}", addr.to_string());
        Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("Content-Type", "text/plain")
            .body("Unauthorized".into())
            .unwrap()
    }
}
