mod secret;

use crate::AppState;
use axum::{routing::get, Router};

pub(super) fn get_secret_router() -> Router<AppState> {
    Router::new().route("/{*key}", get(secret::handle_get).post(secret::handle_post).delete(secret::handle_delete))
}
