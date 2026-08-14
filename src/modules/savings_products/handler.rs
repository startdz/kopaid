use actix_web::{HttpResponse, web};
use sqlx::PgPool;

use crate::{
    errors::AppError,
    infrastructure::auth::authorization::require_permission,
    middleware::current_user::CurrentUser,
    modules::savings_products::{dto::CreateSavingsProductRequest, service},
};

pub async fn list_savings_products(
    current_user: CurrentUser,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    require_permission(pool.get_ref(), current_user.id, "saving_product:read").await?;
    let products = service::list_savings_products(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "data": products
    })))
}

pub async fn create_savings_products(
    current_user: CurrentUser,
    pool: web::Data<PgPool>,
    body: web::Json<CreateSavingsProductRequest>,
) -> Result<HttpResponse, AppError> {
    require_permission(pool.get_ref(), current_user.id, "saving_product:create").await?;
    let product = service::create_savings_product(pool.get_ref(), &body).await?;
    Ok(HttpResponse::Created().json(serde_json::json!({
        "data": product
    })))
}
