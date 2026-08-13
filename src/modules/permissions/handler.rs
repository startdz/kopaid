use crate::{
    errors::AppError::{self, Validation},
    infrastructure::auth::authorization::require_permission,
    middleware::current_user::CurrentUser,
    modules::permissions::{dto::CreatePermissionRequest, service},
};
use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use validator::Validate;

pub async fn list_permissions(
    current_user: CurrentUser,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    require_permission(pool.get_ref(), current_user.id, "permission:read").await?;
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
    require_permission(pool.get_ref(), current_user.id, "permission:create").await?;
    body.validate().map_err(|error| Validation(error))?;
    let permission = service::create_permission(pool.get_ref(), &body).await?;
    Ok(HttpResponse::Created().json(serde_json::json!({
        "data": permission
    })))
}
