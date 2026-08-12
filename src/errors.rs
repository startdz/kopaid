use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum AppError {
    #[display("Database Error")]
    Database,

    #[display("Username or Email already exists")]
    DuplicateUser,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            Self::DuplicateUser => HttpResponse::Conflict().json(serde_json::json!({
                "message": self.to_string()
            })),

            Self::Database => HttpResponse::InternalServerError().json(serde_json::json!({
                "message": self.to_string()
            })),
        }
    }
}
