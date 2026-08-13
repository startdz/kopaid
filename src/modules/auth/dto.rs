use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(
        email(message = "Format email tidak valid"),
        length(min = 1, message = "Email harus di isi")
    )]
    pub email: String,

    #[validate(length(min = 1, message = "Password harus di isi"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
}
