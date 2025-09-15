use sqlx::MySqlPool;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaystackWebhook {
    pub event: String,
    pub data: WebhookData,
}

#[derive(Debug, Deserialize)]
pub struct WebhookData {
    pub reference: String,
    pub status: String,
}

pub async fn handle_webhook(db: &MySqlPool, payload: PaystackWebhook) -> anyhow::Result<()> {
    let status = match payload.data.status.as_str() {
        "success" => "Completed",
        "failed" => "Failed",
        _ => "Processing",
    };

    sqlx::query("UPDATE transactions SET status = ? WHERE reference = ?")
        .bind(status)
        .bind(&payload.data.reference)
        .execute(db)
        .await?;

    Ok(())
}
