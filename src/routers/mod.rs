use crate::configuration::Server;
use crate::AppState;

pub mod axum;

pub async fn main(server: Server, app_state: AppState) {
    axum::axum_server(server, app_state).await;
}
