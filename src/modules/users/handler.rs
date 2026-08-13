use super::{dto::CreateUserRequest, service};
use crate::{
    errors::AppError, middleware::current_user::CurrentUser, modules::users::dto::AssignRoleRequest,
};
use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

// tetap mengadopsi current user agar menandakan aja kalo handler ini butuh authentication.
// tapi middleware current user ini bakal akan berguna jika sudah menerapkan permission dan role.
// untuk saat ini gunakan dulu _current_user untuk adopsi pertama kali.
pub async fn list_users(_current_user: CurrentUser, pool: web::Data<PgPool>) -> HttpResponse {
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

// disini juga menggunakan middleware current_user
// akan lebih relevan jika sudah menerapkan role & permission
// untuk saat ini adopsi simple dahulu
// penggunaan current_user ini untuk audit log kedepannya.
pub async fn create_user(
    current_user: CurrentUser,
    pool: web::Data<PgPool>,
    body: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, AppError> {
    println!("Created by user: {}", current_user.id);

    // validate body
    body.validate()
        .map_err(|error| AppError::Validation(error))?;

    let user = service::create_user(pool.get_ref(), &body).await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "data": user
    })))
}

pub async fn assign_role(
    current_user: CurrentUser,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<AssignRoleRequest>,
) -> Result<HttpResponse, AppError> {
    println!("Updated by user: {}", current_user.id);
    service::assign_role(pool.get_ref(), path.into_inner(), body.role_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
