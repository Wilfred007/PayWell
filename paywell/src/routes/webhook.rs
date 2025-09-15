use std::sync::Arc;
use axum::{extract::State, http::HeaderMap, body::Bytes, http::StatusCode};
use serde::Deserialize;
use crate::state::AppState;
use crate::utils::signature::verify_paystack_signature;
use crate::repo::transaction_repo::TransactionRepo;

#[derive(Deserialize)]
struct PaystackEvent {
    event: String,
    data: serde_json::Value,
}

pub async fn webhook_handler(
    State(app_state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    let body_slice = body.as_ref();

    // ✅ use app_state, not state
    if !verify_paystack_signature(&headers, body_slice, &app_state.paystack_secret) {
        tracing::warn!("invalid webhook signature");
        return StatusCode::UNAUTHORIZED;
    }

    let event: PaystackEvent = match serde_json::from_slice(body_slice) {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("invalid webhook payload: {:?}", e);
            return StatusCode::BAD_REQUEST;
        }
    };

    if event.event.starts_with("transfer.") {
        if let Some(reference) = event.data.get("reference").and_then(|v| v.as_str()) {
            let paystack_status = event.data.get("status").and_then(|v| v.as_str()).unwrap_or("");
            let new_status = match paystack_status {
                "success" | "completed" => "Completed",
                "failed" => "Failed",
                _ => "Processing",
            };

            if let Err(e) = TransactionRepo::update_status_and_meta(
                &app_state.db,
                reference,
                new_status,
                None,
                None,
            )
            .await
            {
                tracing::error!("failed to update tx from webhook: {:?}", e);
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        }
    }

    StatusCode::OK
}
