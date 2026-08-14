use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateSavingsProductRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub min_amount: BigDecimal,
    pub max_amount: Option<BigDecimal>,
}

#[derive(Debug, Serialize)]
pub struct SavingsProductResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub min_amount: BigDecimal,
    pub max_amount: Option<BigDecimal>,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
