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
    let uri = request.uri().path();
    let method = request.method().as_str();
    let mut auth_token: Option<String> = None;
    if let Some(header) = request.headers().get("Authorization") {
        auth_token = Some(header.to_str().unwrap().to_string());
    }
    let authentication = app_state.authentication.clone();
    let is_authorized =
        authentication.is_authorized(auth_token, method.to_string(), uri.to_string());
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
