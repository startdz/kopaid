use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePermissionRequest {
    #[validate(length(min = 4, max = 150, message = "Min. 4 karakter / Maks. 150 Karakter"))]
    pub name: String,

    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PermissionResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}
