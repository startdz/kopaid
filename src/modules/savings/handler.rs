use actix_web::{HttpResponse, web};
use sqlx::PgPool;

use crate::{
    errors::AppError,
    infrastructure::auth::authorization::require_permission,
    middleware::current_user::CurrentUser,
    modules::{
        savings::{dto::CreateSavingsRequest, service},
        savings_transactions::dto::CreateSavingsTransactionRequest,
    },
};

pub async fn list_savings(
    current_user: CurrentUser,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    require_permission(pool.get_ref(), current_user.id, "savings:read").await?;
    let savings = service::list_savings(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({"data": savings})))
}

pub async fn create_savings(
    current_user: CurrentUser,
    pool: web::Data<PgPool>,
    body: web::Json<CreateSavingsRequest>,
) -> Result<HttpResponse, AppError> {
    require_permission(pool.get_ref(), current_user.id, "savings:create").await?;
    let saving = service::create_savings(pool.get_ref(), &body).await?;
    Ok(HttpResponse::Created().json(serde_json::json!({"data": saving})))
}

pub async fn create_savings_transaction(
    current_user: CurrentUser,
    pool: web::Data<PgPool>,
    body: web::Json<CreateSavingsTransactionRequest>,
) -> Result<HttpResponse, AppError> {
    require_permission(pool.get_ref(), current_user.id, "savings:transaction").await?;

    let transaction =
        service::create_savings_transaction(pool.get_ref(), current_user.id, &body).await?;

    Ok(HttpResponse::Created().json(transaction))
}
