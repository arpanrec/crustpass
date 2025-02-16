mod authentication;
mod secret;

use crate::{configuration::Server, routers::authentication::auth_layer, AppState};
use axum::response::IntoResponse;
use axum::{
    http::Response,
    middleware,
    routing::{any, get},
    serve, Router,
};
use std::net::SocketAddr;
use tracing::info;

fn get_secret_router() -> Router<AppState> {
    Router::new().route(
        "/{*key}",
        get(secret::handle_get).post(secret::handle_post).delete(secret::handle_delete),
    )
}

async fn handle_any() -> Response<String> {
    Response::new("".to_string())
}
async fn handle_health() -> impl IntoResponse {
    Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body("OK".to_string())
        .unwrap()
}

pub async fn axum_server(server: Server, app_state: AppState) {
    let addr: SocketAddr = server.socket_addr.parse().unwrap();
    let app = Router::new()
        .nest("/secret", get_secret_router())
        .route("/{*key}", any(handle_any))
        .route("/health", any(handle_health))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_layer))
        .with_state(app_state);
    info!("Starting server on: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
