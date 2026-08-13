use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRoleRequest {
    #[validate(length(min = 3, max = 25, message = "Min. 3 Karakter / Maks. 25 Karakter"))]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RoleResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}
