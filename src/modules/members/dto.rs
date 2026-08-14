use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMemberRequest {
    pub full_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MemberResponse {
    pub id: Uuid,
    pub member_number: String,
    pub full_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub joined_at: time::OffsetDateTime,
}
