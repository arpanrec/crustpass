use crate::{configuration::Server, AppState};
use axum::{http::Response, response::IntoResponse, routing::any, serve, Router};
use std::net::SocketAddr;
use tracing::info;

pub async fn axum_server(server: Server, app_state: AppState) {
    let addr: SocketAddr = server.socket_addr.parse().unwrap();
    let app = Router::new().route("/{*key}", any(handle_any)).with_state(app_state);
    info!("Starting server on: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

async fn handle_any() -> impl IntoResponse {
    Response::new("".to_string())
}
