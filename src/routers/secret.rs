use crate::AppState;
use axum::extract::Request;
use axum::response::IntoResponse;
use axum::{
    extract::{Path, State},
    http::{Response, StatusCode},
};
use tracing::{debug, info};

pub async fn secret(
    Path(path): Path<String>,
    State(state): State<AppState>,
    request: Request,
) -> Result<impl IntoResponse, crate::routers::ServerError> {
    info!("Received get request for key: {}", path);
    let method = request.method();
    let mut storage = state.physical;

    match method.as_str() {
        "GET" => {
            let value = storage
                .read(&path)
                .await
                .map_err(|e| crate::routers::ServerError::InternalServerError(e.to_string()));
            debug!("Read value: {:?}", value);
            if let Ok(Some(value)) = value {
                debug!("Found value: {}", value);
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "text/plain")
                    .body(value)
                    .expect("Failed to send response"))
            } else {
                debug!("Not Found");
                Err(crate::routers::ServerError::NotFound("Key not found".to_string()))
            }
        }
        // "POST" => {
        //     let body = request.body_string().await.unwrap();
        //     debug!("Received body: {}", body);
        //     storage.write(&path, &body).await;
        //     Response::builder()
        //         .status(StatusCode::CREATED)
        //         .header("Content-Type", "text/plain")
        //         .body("".to_string())
        //         .expect("Failed to send response")
        // }
        // "DELETE" => {
        //     storage.delete(&path).await;
        //     Response::builder()
        //         .status(StatusCode::NO_CONTENT)
        //         .header("Content-Type", "text/plain")
        //         .body("".to_string())
        //         .expect("Failed to send response")
        // }
        _ => Err(crate::routers::ServerError::MethodNotAllowed("Method not allowed".to_string())),
    }
}

// let builder = Response::builder();
// if let Some(value) = value {
// debug!("Found value: {}", value);
// builder
// .status(StatusCode::OK)
// .header("Content-Type", "text/plain")
// .body(value)
// .expect("Failed to send response")
// } else {
// debug!("Not Found");
// builder
// .status(StatusCode::NOT_FOUND)
// .header("Content-Type", "text/plain")
// .body("".to_string())
// .expect("Failed to send response")
// }
//
// pub async fn handle_post(
//     Path(path): Path<String>,
//     State(state): State<AppState>,
//     body: String,
// ) -> Response<String> {
//     info!("Received post request for key: {}", path);
//     debug!("Received body: {}", body);
//     let mut storage = state.physical;
//     storage.write(&path, &body).await;
//     Response::builder()
//         .status(StatusCode::CREATED)
//         .header("Content-Type", "text/plain")
//         .body("".to_string())
//         .expect("Failed to send response")
// }
//
// pub async fn handle_delete(
//     Path(path): Path<String>,
//     State(state): State<AppState>,
// ) -> Response<String> {
//     info!("Received delete request for key: {}", path);
//     let mut storage = state.physical;
//     storage.delete(&path).await;
//     Response::new("".to_string())
// }
