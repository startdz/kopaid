use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError, infrastructure::auth::authorization::require_permission,
    middleware::current_user::CurrentUser, modules::savings_transactions::service,
};

pub async fn list_savings_transactions(
    current_user: CurrentUser,
    pool: web::Data<PgPool>,
    savings_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    require_permission(pool.get_ref(), current_user.id, "savings:read").await?;
    let transactions =
        service::list_savings_transactions(pool.get_ref(), savings_id.into_inner()).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "data": transactions
    })))
}
