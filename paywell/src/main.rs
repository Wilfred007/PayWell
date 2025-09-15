
use axum::Router;
use std::sync::Arc;
use sqlx::mysql::MySqlPoolOptions;
use dotenvy::dotenv;
use reqwest::Client;

mod config;
mod routes;
mod services;
mod utils;
mod repo;
mod models;
mod state;

use crate::state::AppState;
use routes::create_router;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Load env variables
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let paystack_base_url = std::env::var("PAYSTACK_BASE_URL").expect("PAYSTACK_BASE_URL must be set");
    let paystack_secret = std::env::var("PAYSTACK_SECRET_KEY").expect("PAYSTACK_SECRET_KEY must be set");
    let bind_addr = std::env::var("BIND_ADDR").unwrap_or("0.0.0.0:8080".to_string());

    // Init HTTP client
    let http = Client::new();

    // Setup DB pool
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connected to database!");
            pool
        }
        Err(err) => {
            eprintln!("❌ Failed to connect to database: {err:?}");
            std::process::exit(1);
        }
    };

    // Build router with shared state
    let app_state = Arc::new(AppState {
        db: pool,
        jwt_secret,
        http,
        paystack_secret,
        paystack_base_url,
    });

    let app = create_router(app_state);

    // Start server
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("Could not bind to address");

    println!("🚀 Server running at http://{bind_addr}");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
