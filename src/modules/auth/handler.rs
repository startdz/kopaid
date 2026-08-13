use actix_web::{HttpResponse, web};
use sqlx::PgPool;

use crate::config::Config;
use crate::modules::auth::dto::LoginResponse;

use super::dto::LoginRequest;
use super::service;

pub async fn login(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, crate::errors::AppError> {
    let access_token = service::login(pool.get_ref(), &config.jwt_secret, &body).await?;
    let response = LoginResponse { access_token };

    Ok(HttpResponse::Ok().json(response))
}
