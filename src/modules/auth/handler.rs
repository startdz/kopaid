use super::dto::LoginRequest;
use super::service;
use crate::config::Config;
use crate::errors::AppError;
use crate::modules::auth::dto::LoginResponse;
use actix_web::{HttpResponse, web};
use sqlx::PgPool;

pub async fn login(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let access_token = service::login(pool.get_ref(), &config.jwt_secret, &body).await?;
    let response = LoginResponse { access_token };

    Ok(HttpResponse::Ok().json(response))
}
