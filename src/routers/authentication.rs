use crate::AppState;
use axum::{
    extract::{ConnectInfo, Request, State},
    http::{Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use std::net::SocketAddr;
use tracing::info;

pub async fn auth_layer(
    State(app_state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    let authentication = app_state.authentication.clone();
    let mut is_authorized = false;
    let uri = request.uri().path();
    let method = request.method().as_str();
    if let Some(header) = request.headers().get("Authorization") {
        if let Ok(value) = header.to_str() {
            is_authorized = authentication.is_authenticate(value, method, uri);
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
