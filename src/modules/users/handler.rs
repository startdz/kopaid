use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use validator::Validate;

use super::{dto::CreateUserRequest, service};

pub async fn list_users(pool: web::Data<PgPool>) -> HttpResponse {
    match service::list_users(pool.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(serde_json::json!({
            "data": users
        })),

        Err(error) => {
            eprintln!("Failed to fetch users: {error}");

            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Failed to fetch users"
            }))
        }
    }
}

pub async fn create_user(
    pool: web::Data<PgPool>,
    body: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, crate::errors::AppError> {
    // validate body
    body.validate()
        .map_err(|error| crate::errors::AppError::Validation(error))?;

    let user = service::create_user(pool.get_ref(), &body).await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "data": user
    })))
}
