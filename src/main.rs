mod auth;
mod physical;
mod routers;
mod settings;

use crate::routers::get_secret_router;
use auth::Authentication;
use physical::Physical;

use axum::{
    extract::{ConnectInfo, Request, State},
    http::{Response, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing, serve, Router,
};

use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Debug)]
struct AppState {
    physical: Physical,
    authentication: Authentication,
}

// #[tokio::main]
fn main() {
    println!("Starting Application...");
    let configuration = settings::load_configuration();
    println!("Server configuration: {:?}", configuration);
    let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap();
    rt.block_on(async {
        axum_server(configuration).await;
    });
}

async fn axum_server(configuration: settings::Configuration) {
    let physical = Physical::new(configuration.physical.clone());
    let authentication = Authentication::new(configuration.authentication.clone());
    let app_state = AppState { physical, authentication };
    let addr: SocketAddr = configuration.socket_addr.parse().unwrap();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .nest("/secret", get_secret_router())
        .route("/{*key}", routing::any(handle_any))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_layer))
        .with_state(app_state);
    info!("Starting server on: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

async fn handle_any() -> Response<String> {
    Response::new("".to_string())
}

async fn auth_layer(
    State(app_state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    let authentication = app_state.authentication.clone();
    let mut is_authorized = false;
    if let Some(header) = request.headers().get("Authorization") {
        if let Ok(value) = header.to_str() {
            is_authorized = authentication.is_authenticate(value);
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
