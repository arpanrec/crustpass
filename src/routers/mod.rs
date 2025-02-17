mod authentication;
mod secret;

use crate::{configuration::Server, routers::authentication::auth_layer, AppState};
use axum::{
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::any,
    serve, Router,
};

use std::{fmt::Display, net::SocketAddr};
use tracing::info;

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
        match self {
            ServerError::NotFound(e) => {
                (StatusCode::NOT_FOUND, format!("Resource Not Found: {}", e)).into_response()
            }
            ServerError::InternalServerError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal Server Error: {}", e))
                    .into_response()
            }
            ServerError::Unauthorized(e) => {
                (StatusCode::UNAUTHORIZED, format!("Unauthorized: {}", e)).into_response()
            }
            ServerError::MethodNotAllowed(e) => {
                (StatusCode::METHOD_NOT_ALLOWED, format!("Method Not Allowed: {}", e))
                    .into_response()
            }
            ServerError::RouterError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Router Error: {}", e)).into_response()
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
        .with_state(app_state);
    info!("Starting server on: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.map_err(|e| {
        ServerError::RouterError(format!("Unable to bind address: {}, error: {}", addr, e))
    })?;
    serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .map_err(|e| ServerError::RouterError(format!("Error serving: {}", e)))
}
