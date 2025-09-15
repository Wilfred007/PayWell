use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::sync::Arc;

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct ResolveRequest {
    pub bank_account_number: String,
    pub bank_code: String,
}

#[derive(Debug, Serialize)]
pub struct ResolveResponse {
    pub account_number: String,
    pub account_name: String,
    pub bank_code: String,
}

pub async fn resolve_account(
    State(app_state): State<Arc<AppState>>,   // ✅ take Arc<AppState>
    Json(payload): Json<ResolveRequest>,
) -> Result<Json<ResolveResponse>, (axum::http::StatusCode, String)> {
    let client = Client::new();

    let url = format!(
        "https://api.paystack.co/bank/resolve?account_number={}&bank_code={}",
        payload.bank_account_number, payload.bank_code
    );

    let res = client
        .get(&url)
        .bearer_auth(&app_state.paystack_secret)   // ✅ pull from AppState
        .send()
        .await
        .map_err(|_| (axum::http::StatusCode::BAD_REQUEST, "Failed to call Paystack".to_string()))?;

    let body: serde_json::Value = res
        .json()
        .await
        .map_err(|_| (axum::http::StatusCode::BAD_REQUEST, "Invalid response from Paystack".to_string()))?;

    if !body["status"].as_bool().unwrap_or(false) {
        return Err((axum::http::StatusCode::BAD_REQUEST, "Could not resolve account".to_string()));
    }

    Ok(Json(ResolveResponse {
        account_number: body["data"]["account_number"].as_str().unwrap_or("").to_string(),
        account_name: body["data"]["account_name"].as_str().unwrap_or("").to_string(),
        bank_code: payload.bank_code,
    }))
}
