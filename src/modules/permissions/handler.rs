use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    errors::AppError::{self, Validation},
    middleware::current_user::CurrentUser,
    modules::permissions::{dto::CreatePermissionRequest, service},
};

pub async fn list_permissions(
    _current_user: CurrentUser,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let permissions = service::list_permissions(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "data": permissions
    })))
}

pub async fn create_permission(
    current_user: CurrentUser,
    pool: web::Data<PgPool>,
    body: web::Json<CreatePermissionRequest>,
) -> Result<HttpResponse, AppError> {
    println!("Created permission by user: {}", current_user.id);
    body.validate().map_err(|error| Validation(error))?;

    let permission = service::create_permission(pool.get_ref(), &body).await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "data": permission
    })))
}
