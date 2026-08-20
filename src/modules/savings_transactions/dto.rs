use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavingsTransactionType {
    Deposit,
    Withdrawal,
}

#[derive(Debug, Deserialize)]
pub struct CreateSavingsTransactionRequest {
    pub savings_id: Uuid,
    pub transaction_type: SavingsTransactionType,
    pub amount: BigDecimal,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SavingsTransactionResponse {
    pub id: Uuid,
    pub savings_id: Uuid,
    pub transaction_type: String,
    pub amount: BigDecimal,
    pub reference_number: Option<String>,
    pub description: Option<String>,
    pub created_by: Uuid,
    pub created_at: OffsetDateTime,
}
