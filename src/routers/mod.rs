mod authentication;
mod secret;

use crate::{configuration::Server, routers::authentication::auth_layer, AppState};
use axum::{
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{any, get},
    serve, Router,
};

use std::{fmt::Display, net::SocketAddr};
use tracing::info;

#[derive(Debug)]
enum ServerError {
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
        }
    }
}

fn get_secret_router() -> Router<AppState> {
    Router::new().route("/{*key}", get(secret::secret))
}

// async fn handle_any() -> Result<impl IntoResponse, crate::routers::ServerError> {
//     Err(crate::routers::ServerError::MethodNotAllowed("Method not allowed".to_string()))
// }
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
        // .route("/{*key}", any(handle_any))
        .route("/health", any(handle_health))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_layer))
        .with_state(app_state);
    info!("Starting server on: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
