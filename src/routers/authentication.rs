use crate::AppState;
use axum::{
    extract::{ConnectInfo, Request, State},
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
) -> Result<impl IntoResponse, crate::routers::ServerError> {
    let uri = request.uri().path();
    let method = request.method().as_str();
    let mut auth_token: Option<String> = None;
    if let Some(header) = request.headers().get("Authorization") {
        auth_token = Some(
            header
                .to_str()
                .map_err(|e| {
                    crate::routers::ServerError::Unauthorized(format!("Error parsing token: {}", e))
                })?
                .to_string(),
        );
    }
    let authentication = app_state.authentication.clone();
    let is_authorized =
        authentication.is_authorized(auth_token, method.to_string(), uri.to_string());
    if is_authorized {
        Ok(next.run(request).await.into_response())
    } else {
        info!("Unauthorized request from: {:?}", addr.to_string());
        Err(crate::routers::ServerError::Unauthorized("Invalid or Missing token".to_string()))
    }
}
