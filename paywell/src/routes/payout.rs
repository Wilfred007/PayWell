// use axum::{Json, extract::State};
// use sqlx::MySqlPool;
// use crate::models::models::{CreatePayout, Transaction};
// use crate::services::payout_services::initiate_payout;
// use crate::config::Config;
// use crate::routes::auth;
// use crate::routes::auth_middleware::AuthUser;
// // use crate::routes::auth_middleware::AuthUser;

// pub async fn create_payout(
//     State(db): State<MySqlPool>,
//     AuthUser { user_id }: AuthUser,
//     Json(payload): Json<CreatePayout>,
// ) -> Json<Transaction> {
//     let config = Config::from_env();
//     let tx = initiate_payout(&db, payload, &user_id, &config.paystack_secret)
//         .await
//         .unwrap();
//     Json(tx)
// }


use axum::{Json, extract::State};
use std::sync::Arc;
use crate::models::models::{CreatePayout, Transaction};
use crate::services::payout_services::initiate_payout;
use crate::config::Config;
use crate::routes::auth_middleware::AuthUser;
use crate::AppState;

pub async fn create_payout(
    State(app_state): State<Arc<AppState>>,   // 👈 Use Arc<AppState>
    AuthUser { user_id }: AuthUser,
    Json(payload): Json<CreatePayout>,
) -> Json<Transaction> {
    let config = Config::from_env();

    // now pull db out of AppState
    let tx = initiate_payout(&app_state.db, payload, &user_id, &config.paystack_secret)
        .await
        .unwrap();

    Json(tx)
}
