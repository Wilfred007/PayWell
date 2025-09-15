use axum::{extract::State, Json};
use serde::Deserialize;
use sqlx::MySqlPool;
use std::sync::Arc;
use crate::AppState;
use crate::services::auth_services::{hash_password, generate_jwt, verify_password};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn register(
    State(state): State<Arc<AppState>>, 
    Json(body): Json<RegisterRequest>
) -> Json<String> {
    let db: &MySqlPool = &state.db;

    let id = uuid::Uuid::new_v4().to_string();
    let hashed = hash_password(&body.password).expect("failed to hash password");

    sqlx::query("INSERT INTO users (id, email, password_hash) VALUES (?, ?, ?)")
        .bind(&id)
        .bind(&body.email)
        .bind(&hashed)
        .execute(db)
        .await
        .expect("failed to insert user");

    Json("User registered".into())
}

pub async fn login(
    State(state): State<Arc<AppState>>, 
    Json(body): Json<LoginRequest>
) -> Json<String> {
    let db: &MySqlPool = &state.db;

    let user = sqlx::query!("SELECT id, password_hash FROM users WHERE email = ?", body.email)
        .fetch_one(db)
        .await
        .unwrap();

    if verify_password(&body.password, &user.password_hash) {
        let token = generate_jwt(&user.id, &state.jwt_secret)
            .expect("Failed to generate token");
        Json(token)
    } else {
        Json("Invalid credentials".into())
    }
}
