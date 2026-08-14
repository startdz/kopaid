use actix_web::{HttpResponse, web};
use sqlx::PgPool;

use crate::{
    errors::AppError,
    infrastructure::auth::authorization::require_permission,
    middleware::current_user::CurrentUser,
    modules::members::{dto::CreateMemberRequest, service},
};

pub async fn list_members(
    current_user: CurrentUser,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    require_permission(pool.get_ref(), current_user.id, "member:read").await?;

    let members = service::list_members(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "data": members
    })))
}

pub async fn create_member(
    current_user: CurrentUser,
    pool: web::Data<PgPool>,
    body: web::Json<CreateMemberRequest>,
) -> Result<HttpResponse, AppError> {
    require_permission(pool.get_ref(), current_user.id, "member:create").await?;

    let member = service::create_member(pool.get_ref(), &body).await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "data": member
    })))
}
