mod authentication;
mod logging;
mod secret;

use crate::{configuration::Server, routers::authentication::auth_layer, AppState};
use axum::{
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::any,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use std::{fmt::Display, net::SocketAddr};
use tracing::{info, warn};

#[derive(Debug)]
pub enum ServerError {
    RouterError(String),
    NotFound(String),
    InternalServerError(String),
    Unauthorized(String),
    MethodNotAllowed(String),
}
impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::NotFound(e) => write!(f, "Not Found: {}", e),
            ServerError::InternalServerError(e) => write!(f, "Internal Server Error: {}", e),
            ServerError::Unauthorized(e) => write!(f, "Unauthorized: {}", e),
            ServerError::MethodNotAllowed(e) => write!(f, "Method Not Allowed: {}", e),
            ServerError::RouterError(e) => write!(f, "Router Error: {}", e),
        }
    }
}
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let random = rand::random::<u32>();
        match self {
            ServerError::NotFound(e) => {
                warn!("{} Resource Not Found: {}", random, e);
                (StatusCode::NOT_FOUND, format!("Resource Not Found: Check logs for {}", random))
                    .into_response()
            }
            ServerError::InternalServerError(e) => {
                warn!("{} Resource Not Found: {}", random, e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Internal Server Error: Check logs for {}", random),
                )
                    .into_response()
            }
            ServerError::Unauthorized(e) => {
                warn!("{} Unauthorized: {}", random, e);
                (StatusCode::UNAUTHORIZED, format!("Unauthorized: Check logs for {}", random))
                    .into_response()
            }
            ServerError::MethodNotAllowed(e) => {
                warn!("{} Method Not Allowed: {}", random, e);
                (
                    StatusCode::METHOD_NOT_ALLOWED,
                    format!("Method Not Allowed: Check logs for {}", random),
                )
                    .into_response()
            }
            ServerError::RouterError(e) => {
                panic!("Router Error, RouterErrors are not meant to be returned: {}", e)
            }
        }
    }
}

fn get_secret_router() -> Router<AppState> {
    Router::new().route("/{*key}", any(secret::secret))
}

async fn handle_any() -> Result<impl IntoResponse, ServerError> {
    Err::<Response, ServerError>(ServerError::MethodNotAllowed("Unknown Resource".to_string()))
        as Result<_, ServerError>
}
async fn handle_health() -> Result<impl IntoResponse, ServerError> {
    Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body("OK".to_string())
        .map_err(|e| ServerError::InternalServerError(format!("Error creating response: {}", e)))
}
//noinspection HttpUrlsUsage
pub async fn axum_server(server: Server, app_state: AppState) -> Result<(), ServerError> {
    let addr: SocketAddr = server.socket_addr.parse().map_err(|e| {
        ServerError::RouterError(format!(
            "Unable to parse address: {}, error: {}",
            server.socket_addr, e
        ))
    })?;
    let app = Router::new()
        .nest("/secret", get_secret_router())
        .route("/{*key}", any(handle_any))
        .route("/health", any(handle_health))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_layer))
        .with_state(app_state)
        .layer(middleware::from_fn(logging::print_request_response));

    if let Some(server_tls) = server.tls {
        let config =
            RustlsConfig::from_pem(server_tls.cert.into_bytes(), server_tls.key.into_bytes())
                .await
                .map_err(|e| {
                    ServerError::RouterError(format!("Error creating rustls TLS config: {}", e))
                })?;
        info!("Starting server with https://{}", addr);
        axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .map_err(|e| {
                ServerError::RouterError(format!("Error serving without rustls: {}", e.to_string()))
            })
    } else {
        info!("Starting server on http://{}", addr);
        axum_server::bind(addr)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .map_err(|e| {
                ServerError::RouterError(format!("Error serving without rustls: {}", e.to_string()))
            })
    }
}
