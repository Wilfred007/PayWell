use std::sync::Arc;
use axum::{routing::{get, post}, Router};
use crate::AppState;
// use crate::routes::auth;

use axum::{response::IntoResponse, Json};
use serde_json::json;

pub mod payout;
pub mod webhook;
pub mod auth;
pub mod auth_middleware;
pub mod resolve_routes;

/// Build the router with all routes
pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/healthcheck", get(health_check_handler))
        .route("/api/v1/payouts", post(payout::create_payout))
        .route("/api/v1/resolve-account", post(resolve_routes::resolve_account))
        .route("/api/v1/paystack/webhook", post(webhook::webhook_handler))
        .route("/api/v1/auth/register", post(auth::register))
        .route("/api/v1/auth/login", post(auth::login))
        .with_state(app_state)
}



/// Healthcheck endpoint
pub async fn health_check_handler() -> impl IntoResponse {
    let json_response = json!({
        "status": "ok",
        "message": "API Service"
    });
    Json(json_response)
}
