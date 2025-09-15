use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Transaction {
    pub id: String,
    pub user_id: String,
    pub amount: f64,
    pub bank_account_number: String,
    pub bank_code: String,
    pub reference: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePayout {
    pub amount: f64,
    pub bank_account_number: String,
    pub bank_code: String,
    pub tx_reference: String,
}
