use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use serde_json::json;
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;

use crate::{
    config::ServerConfig,
    state::{self, AppState},
    util::shutdown_signal, storage::handle::get_handle_manager,
};
use atproto_core::error::Result;

pub(crate) async fn cmd_server(server_config: ServerConfig) -> Result<()> {
    println!("Server {}:{}", server_config.address, server_config.port);

    let handle_manager = get_handle_manager(&server_config.handle_manager_type);

    let shared_state = state::AppState(Arc::new(state::InnerState::new(server_config.clone(), handle_manager)));

    {
        // TODO: Don't do this until it's time.
        shared_state.set_ready();
        shared_state.set_alive();
        shared_state.set_started();
    }

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

enum ReadyError {
    NotReady,
    NotStarted,
    NotAlive,
}

impl IntoResponse for ReadyError {
    fn into_response(self) -> Response {
        let body = match self {
            ReadyError::NotReady => "service is not ready",
            ReadyError::NotAlive => "service is not alive",
            ReadyError::NotStarted => "service is not started",
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

async fn handle_index(State(state): State<AppState>) -> impl IntoResponse {
    Json(json!({"version": state.config.version}))
}

async fn handle_check_started(State(state): State<AppState>) -> impl IntoResponse {
    if state.started.load(std::sync::atomic::Ordering::Relaxed) {
        return Ok(Json(json!({"version": state.config.version})));
    }
    Err(ReadyError::NotStarted)
}

async fn handle_check_alive(State(state): State<AppState>) -> impl IntoResponse {
    if state.alive.load(std::sync::atomic::Ordering::Relaxed) {
        return Ok(Json(json!({"version": state.config.version})));
    }
    Err(ReadyError::NotAlive)
}

async fn handle_check_ready(State(state): State<AppState>) -> impl IntoResponse {
    if state.ready.load(std::sync::atomic::Ordering::Relaxed) {
        return Ok(Json(json!({"version": state.config.version})));
    }
    Err(ReadyError::NotReady)
}

async fn handle_well_known_jwks(State(state): State<AppState>) -> impl IntoResponse {
    let jwks = jsonwebtoken::jwk::JwkSet {
        keys: state.config.signing_keys.clone(),
    };
    Json(jwks)
}

async fn handle_com_atproto_handle_resolve(State(state): State<AppState>) -> impl IntoResponse {
    let jwks = jsonwebtoken::jwk::JwkSet {
        keys: state.config.signing_keys.clone(),
    };
    Json(jwks)
}

pub(crate) fn build_router(shared_state: AppState) -> Router {
    Router::new()
        .route("/", get(handle_index))
        .route("/check/startup", get(handle_check_started))
        .route("/check/liveliness", get(handle_check_alive))
        .route("/check/readiness", get(handle_check_ready))
        .route("/.well-known/jwks.json", get(handle_well_known_jwks))
        .route("/xrpc/com.atproto.handle.resolve", get(handle_com_atproto_handle_resolve))
        .layer(TraceLayer::new_for_http())
        .with_state(shared_state)
}
