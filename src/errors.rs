use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use validator::ValidationErrors;

#[derive(Debug, Display)]
pub enum AppError {
    #[display("Database Error")]
    Database,

    #[display("Username or Email already exists")]
    DuplicateUser,

    #[display("Validation error")]
    Validation(ValidationErrors),

    #[display("Internal Server Error")]
    Internal,

    #[display("Invalid email or password")]
    InvalidCredentials,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            Self::DuplicateUser => HttpResponse::Conflict().json(serde_json::json!({
                "message": self.to_string()
            })),

            Self::Validation(errors) => HttpResponse::BadRequest().json(serde_json::json!({
                "message": "Validation failed",
                "errors": errors
            })),

            Self::Database => HttpResponse::InternalServerError().json(serde_json::json!({
                "message": self.to_string()
            })),

            Self::Internal => HttpResponse::InternalServerError().json(serde_json::json!({
                "message": self.to_string()
            })),

            Self::InvalidCredentials => HttpResponse::Unauthorized().json(serde_json::json!({
                "message": "Invalid email or password"
            })),
        }
    }
}
