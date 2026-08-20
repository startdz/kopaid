use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateSavingsRequest {
    pub member_id: Uuid,
    pub product_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct SavingsResponse {
    pub id: Uuid,
    pub account_number: String,
    pub member_id: Uuid,
    pub product_id: Uuid,
    pub balance: BigDecimal,
    pub is_active: bool,
    pub opened_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
