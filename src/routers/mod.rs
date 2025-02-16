mod auth;
mod secret;

use crate::{auth::Authentication, physical::Physical, routers::auth::auth_layer, settings::Configuration, AppState};
use axum::{http::Response, middleware, routing, routing::get, serve, Router};
use std::net::SocketAddr;
use tracing::info;

fn get_secret_router() -> Router<AppState> {
    Router::new().route("/{*key}", get(secret::handle_get).post(secret::handle_post).delete(secret::handle_delete))
}

async fn handle_any() -> Response<String> {
    Response::new("".to_string())
}

pub async fn axum_server(configuration: Configuration) {
    let physical = Physical::new(configuration.physical.clone());
    let authentication = Authentication::new(configuration.authentication.clone());
    let app_state = AppState { physical, authentication };
    let addr: SocketAddr = configuration.socket_addr.parse().unwrap();
    let app = Router::new()
        .nest("/secret", get_secret_router())
        .route("/{*key}", routing::any(handle_any))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_layer))
        .with_state(app_state);
    info!("Starting server on: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
