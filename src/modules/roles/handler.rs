use crate::{
    errors::AppError::{self, Validation},
    middleware::current_user::CurrentUser,
    modules::roles::{
        dto::{AssignPermissionRequest, CreateRoleRequest},
        service,
    },
};
use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

pub async fn list_roles(
    _current_user: CurrentUser,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let roles = service::list_roles(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "data": roles
    })))
}

pub async fn create_role(
    _current_user: CurrentUser,
    pool: web::Data<PgPool>,
    body: web::Json<CreateRoleRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate().map_err(|error| Validation(error))?;

    let role = service::create_role(pool.get_ref(), &body).await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "data": role
    })))
}

pub async fn assign_permission(
    _current_user: CurrentUser,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<AssignPermissionRequest>,
) -> Result<HttpResponse, AppError> {
    service::assign_permission(pool.get_ref(), path.into_inner(), body.permission_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
