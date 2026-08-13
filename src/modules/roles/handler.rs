use crate::{
    errors::AppError::{self, Validation},
    middleware::current_user::CurrentUser,
    modules::roles::{dto::CreateRoleRequest, service},
};
use actix_web::{HttpResponse, web};
use sqlx::PgPool;
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
