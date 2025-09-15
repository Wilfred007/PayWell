use crate::models::models::Transaction;
use sqlx::{MySql, Executor, MySqlPool, Row};
use sqlx::mysql::MySqlRow;
use anyhow::Result;

pub struct TransactionRepo;

impl TransactionRepo {
    pub async fn find_by_reference(pool: &MySqlPool, reference: &str) -> Result<Option<Transaction>> {
        let rec = sqlx::query_as::<_, Transaction>(
            "SELECT * FROM transactions WHERE reference = ?"
        )
        .bind(reference)
        .fetch_optional(pool)
        .await?;
        Ok(rec)
    }

    pub async fn insert(pool: &MySqlPool,
        reference: &str,
        amount_kobo: i64,
        bank_account_number: &str,
        bank_code: &str,
        recipient_name: &str
    ) -> Result<Transaction> {
        // Try insert -> if duplicate (another process inserted same reference), fetch existing
        let res = sqlx::query(
            r#"INSERT INTO transactions (reference, amount_kobo, bank_account_number, bank_code, recipient_name, status)
               VALUES (?, ?, ?, ?, ?, 'Pending')"#
        )
        .bind(reference)
        .bind(amount_kobo)
        .bind(bank_account_number)
        .bind(bank_code)
        .bind(recipient_name)
        .execute(pool)
        .await;

        match res {
            Ok(_) => {
                let tx = Self::find_by_reference(pool, reference).await?;
                Ok(tx.expect("inserted but cannot fetch"))
            }
            Err(e) => {
                // MySQL duplicate key error code is 1062
                if let sqlx::Error::Database(db_err) = &e {
                    if db_err.code().map(|s| s == "1062").unwrap_or(false) {
                        // someone else already created; fetch it
                        if let Some(existing) = Self::find_by_reference(pool, reference).await? {
                            return Ok(existing);
                        }
                    }
                }
                Err(e.into())
            }
        }
    }

    pub async fn update_status_and_meta(
        pool: &MySqlPool,
        reference: &str,
        status: &str,
        paystack_transfer_id: Option<&str>,
        recipient_code: Option<&str>,
    ) -> Result<()> {
        sqlx::query(
            "UPDATE transactions SET status = ?, paystack_transfer_id = ?, recipient_code = ? WHERE reference = ?"
        )
        .bind(status)
        .bind(paystack_transfer_id)
        .bind(recipient_code)
        .bind(reference)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn fetch_by_reference(pool: &MySqlPool, reference: &str) -> Result<Option<Transaction>> {
        Self::find_by_reference(pool, reference).await
    }
}
