use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::AppError, modules::savings_transactions::dto::SavingsTransactionResponse};

pub async fn list_savings_transactions(
    pool: &PgPool,
    savings_id: Uuid,
) -> Result<Vec<SavingsTransactionResponse>, AppError> {
    let transactions = sqlx::query_as!(
        SavingsTransactionResponse,
        r#"
            SELECT
                id,
                savings_id,
                transaction_type,
                amount,
                reference_number,
                description,
                created_by,
                created_at
            FROM savings_transactions
            WHERE savings_id = $1
            ORDER BY created_at DESC
        "#,
        savings_id
    )
    .fetch_all(pool)
    .await
    .map_err(|error| {
        eprintln!("Failed to list savings transactions: {}", error);
        AppError::Database
    })?;

    Ok(transactions)
}
