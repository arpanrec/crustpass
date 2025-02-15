use axum::{
    extract::{Path, State},
    http::{Response, StatusCode},
};
use tracing::{debug, info};
use crate::AppState;

pub async fn handle_get(Path(path): Path<String>, State(state): State<AppState>) -> Response<String> {
    info!("Received get request for key: {}", path);
    let mut storage = state.physical;
    let value = storage.read(&path).await;
    debug!("Read value: {:?}", value);
    let builder = Response::builder();
    if let Some(value) = value {
        debug!("Found value: {}", value);
        builder
            .status(StatusCode::OK)
            .header("Content-Type", "text/plain")
            .body(value)
            .expect("Failed to send response")
    } else {
        debug!("Not Found");
        builder
            .status(StatusCode::NOT_FOUND)
            .header("Content-Type", "text/plain")
            .body("".to_string())
            .expect("Failed to send response")
    }
}

pub async fn handle_post(Path(path): Path<String>, State(state): State<AppState>, body: String) -> Response<String> {
    info!("Received post request for key: {}", path);
    debug!("Received body: {}", body);
    let mut storage = state.physical;
    storage.write(&path, &body).await;
    Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "text/plain")
        .body("".to_string())
        .expect("Failed to send response")
}

pub async fn handle_delete(Path(path): Path<String>, State(state): State<AppState>) -> Response<String> {
    info!("Received delete request for key: {}", path);
    let mut storage = state.physical;
    storage.delete(&path).await;
    Response::new("".to_string())
}
