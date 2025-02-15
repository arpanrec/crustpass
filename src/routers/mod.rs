mod secret;

use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub(super) fn get_secret_router() -> Router<AppState> {
    Router::new()
        .route("/{*key}", get(secret::handle_get).delete(secret::handle_delete))
        .route("/{*key}", post(secret::handle_post))
}
