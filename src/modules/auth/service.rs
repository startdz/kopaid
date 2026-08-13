use super::dto::LoginRequest;
use crate::{errors::AppError, infrastructure::auth::jwt::generate_access_token};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use sqlx::PgPool;

pub async fn login(
    pool: &PgPool,
    jwt_secret: &str,
    request: &LoginRequest,
) -> Result<String, AppError> {
    let user = sqlx::query!(
        r#"
        SELECT
            id,
            username,
            email,
            password_hash
        FROM users
        WHERE email = $1
        "#,
        request.email
    )
    .fetch_optional(pool)
    .await
    .map_err(|error| {
        eprintln!("Failed to find user: {error}");
        AppError::Database
    })?;

    let Some(user) = user else {
        return Err(AppError::InvalidCredentials);
    };

    let password_hash = PasswordHash::new(&user.password_hash).map_err(|error| {
        eprintln!("Invalid password hash: ${error}");
        AppError::Internal
    })?;

    Argon2::default()
        .verify_password(request.password.as_bytes(), &password_hash)
        .map_err(|_| AppError::InvalidCredentials)?;

    let access_token = generate_access_token(user.id, jwt_secret).map_err(|error| {
        eprintln!("Failed to generate access token: {error}");
        AppError::Internal
    })?;

    Ok(access_token)
}
