mod physical;
use crate::physical::Storage;
use axum::extract::State;
use axum::routing::post;
use axum::{
    body::Body,
    extract,
    extract::Path,
    http,
    http::{Response, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Router,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct MainState {
    storage: Storage,
    auth_conf: String,
    listen: String,
}

// #[tokio::main]
fn main() {
    println!("Starting Secret Squirrel...");
    let storage: Storage = Storage::new();
    let auth_conf = std::env::var("SQ_AUTH_CONF").unwrap_or("admin".to_string());
    let listen = std::env::var("SQ_LISTEN").unwrap_or("0.0.0.0:3000".to_string());
    let shared_state = MainState { storage, auth_conf, listen };
    let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap();
    rt.block_on(async {
        axum_server(shared_state).await;
    });
}

async fn axum_server(shared_state: MainState) {
    println!("Starting server...");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let addr: std::net::SocketAddr = shared_state.listen.parse().unwrap();
    let app = Router::new()
        .route("/secret/{*key}", get(handle_get).delete(handle_delete))
        .route("/secret/{*key}", post(handle_post))
        .route("/{*key}", axum::routing::any(handle_any))
        .layer(middleware::from_fn_with_state(shared_state.clone(), auth_layer))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_get(Path(path): Path<String>, State(state): State<MainState>) -> Response<String> {
    println!("Received request for key: {}", path);
    let mut storage = state.storage;
    let value = storage.read(&path).await;
    println!("Read value: {:?}", value);
    let builder = Response::builder();
    if let Some(value) = value {
        println!("Found value: {}", value);
        builder
            .status(StatusCode::OK)
            .header("Content-Type", "text/plain")
            .body(value)
            .expect("Failed to send response")
    } else {
        println!("Not Found");
        builder
            .status(StatusCode::NOT_FOUND)
            .header("Content-Type", "text/plain")
            .body("".to_string())
            .expect("Failed to send response")
    }
}

async fn handle_post(Path(path): Path<String>, State(state): State<MainState>, body: String) -> Response<String> {
    println!("Received request for key: {}", path);
    println!("Received body: {}", body);
    let mut storage = state.storage;
    storage.write(&path, &body).await;
    Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "text/plain")
        .body("".to_string())
        .expect("Failed to send response")
}

async fn handle_delete(Path(path): Path<String>, State(state): State<MainState>) -> Response<String> {
    println!("Received request for key: {}", path);
    let mut storage = state.storage;
    storage.delete(&path).await;
    Response::new("".to_string())
}

async fn handle_any(Path(path): Path<String>, request: http::Request<Body>) -> Response<String> {
    println!("Received request for lock with method: {:?}", request.method());
    println!("Received request for key: {}", path);
    Response::new("".to_string())
}

async fn auth_layer(State(state): State<MainState>, request: extract::Request, next: Next) -> impl IntoResponse {
    let auth_conf: String = state.auth_conf;
    let mut is_authorized = false;
    if let Some(header) = request.headers().get("Authorization") {
        if let Ok(value) = header.to_str() {
            if value == auth_conf {
                is_authorized = true;
            }
        }
    }
    if is_authorized {
        next.run(request).await.into_response()
    } else {
        println!("Unauthorized request");
        Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("Content-Type", "text/plain")
            .body("Unauthorized".into())
            .unwrap()
    }
}
