use crate::models::models::{CreatePayout, Transaction};
use sqlx::MySqlPool;
use uuid::Uuid;
use reqwest::Client;
use serde_json::json;

pub async fn initiate_payout(
    db: &MySqlPool,
    payload: CreatePayout,
    user_id: &str,
    paystack_secret: &str,
) -> anyhow::Result<Transaction> {
    // Idempotency check
    if let Some(tx) = sqlx::query_as::<_, Transaction>("SELECT * FROM transactions WHERE reference = ?")
        .bind(&payload.tx_reference)
        .fetch_optional(db)
        .await? {
        return Ok(tx);
    }

    // Save initial transaction
    let id = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO transactions (id, user_id, amount, bank_account_number, bank_code, reference, status) VALUES (?, ?, ?, ?, ?, ?, 'Processing')")
        .bind(&id)
        .bind(user_id)
        .bind(payload.amount)
        .bind(&payload.bank_account_number)
        .bind(&payload.bank_code)
        .bind(&payload.tx_reference)
        .execute(db)
        .await?;

    // Call Paystack API
    let client = Client::new();
    let res = client
        .post("https://api.paystack.co/transfer")
        .bearer_auth(paystack_secret)
        .json(&json!({
            "source": "balance",
            "reason": "Payout",
            "amount": (payload.amount * 100.0) as i64,
            "recipient": {
                "type": "nuban",
                "name": "Beneficiary",
                "account_number": payload.bank_account_number,
                "bank_code": payload.bank_code
            },
            "reference": payload.tx_reference
        }))
        .send()
        .await?;

    println!("Paystack response: {:?}", res.text().await?);

    let tx = sqlx::query_as::<_, Transaction>("SELECT * FROM transactions WHERE id = ?")
        .bind(&id)
        .fetch_one(db)
        .await?;

    Ok(tx)
}
