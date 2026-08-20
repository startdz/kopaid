use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use validator::ValidationErrors;

#[derive(Debug, Display)]
pub enum AppError {
    #[display("Database error")]
    Database,

    #[display("Invalid credentials")]
    InvalidCredentials,

    #[display("Conflict")]
    Conflict,

    #[display("Internal Server Error")]
    Internal,

    #[display("Validation Error")]
    Validation(ValidationErrors),

    #[display("Not Found")]
    NotFound,

    #[display("Forbidden")]
    Forbidden,

    #[display("Insufficient balance")]
    InsufficientBalance,

    #[display("Invalid amount")]
    InvalidAmount,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::Database => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "DATABASE_ERROR",
                    "message": "Database error"
                }
            })),

            Self::InvalidCredentials => HttpResponse::Unauthorized().json(serde_json::json!({
                "error": {
                    "code": "INVALID_CREDENTIALS",
                    "message": "Invalid email or password"
                }
            })),

            Self::Conflict => HttpResponse::Conflict().json(serde_json::json!({
                "error": {
                    "code": "CONFLICT",
                    "message": "Resource already exists"
                }
            })),

            Self::NotFound => HttpResponse::NotFound().json(serde_json::json!({
                "error": {
                    "code": "NOT_FOUND",
                    "message": "Resource not found"
                }
            })),

            Self::Internal => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": {
                    "code": "INTERNAL_SERVER_ERROR",
                    "message": "Internal server error"
                }
            })),

            Self::Forbidden => HttpResponse::Forbidden().json(serde_json::json!({
                "error": {
                    "code": "FORBIDDEN",
                    "message": "You do not have permission to perform this action"
                }
            })),

            Self::InvalidAmount => HttpResponse::UnprocessableEntity().json(serde_json::json!({
                "error": {
                    "code": "INVALID_AMOUNT",
                    "message": "Amount must be greater than zero"
                }
            })),

            Self::InsufficientBalance => {
                HttpResponse::UnprocessableEntity().json(serde_json::json!({
                    "error": {
                        "code": "INSUFFICIENT_BALANCE",
                        "message": "Insufficient balance"
                    }
                }))
            }

            Self::Validation(error) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": {
                        "code": "VALIDATION_ERROR",
                        "message": "Validation failed",
                        "details": error
                    }
                }))
            }
        }
    }
}
