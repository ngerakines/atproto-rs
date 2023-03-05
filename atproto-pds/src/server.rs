use axum::{
    extract::{Json, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde_json::json;
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;

use crate::{
    config::ServerConfig,
    state::{self, AppState},
    util::shutdown_signal,
};
use atproto_core::error::Result;

pub(crate) async fn cmd_server(server_config: ServerConfig) -> Result<()> {
    println!("Server {}:{}", server_config.address, server_config.port);

    let shared_state = state::AppState(Arc::new(state::InnerState::new(server_config.clone())));

    let app = build_router(shared_state);

    let listen = format!("{}:{}", server_config.address, server_config.port);

    let socket_addr = listen.parse::<SocketAddr>()?;

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
}

async fn handle_index(State(state): State<AppState>) -> impl IntoResponse {
    Json(json!({"version": state.config.version}))
}

async fn handle_check_started(State(state): State<AppState>) -> impl IntoResponse {
    Json(json!({"version": state.config.version}))
}

async fn handle_check_alive(State(state): State<AppState>) -> impl IntoResponse {
    Json(json!({"version": state.config.version}))
}

async fn handle_check_ready(State(state): State<AppState>) -> impl IntoResponse {
    Json(json!({"version": state.config.version}))
}

pub(crate) fn build_router(shared_state: AppState) -> Router {
    Router::new()
        .route("/", get(handle_index))
        .route("/check/startup", get(handle_check_started))
        .route("/check/liveliness", get(handle_check_alive))
        .route("/check/readiness", get(handle_check_ready))
        .layer(TraceLayer::new_for_http())
        .with_state(shared_state)
}
