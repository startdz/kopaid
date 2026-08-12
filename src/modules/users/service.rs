use super::dto::{CreateUserRequest, UserResponse};
use crate::errors::AppError;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn list_users(pool: &PgPool) -> Result<Vec<UserResponse>, sqlx::Error> {
    let users = sqlx::query_as!(
        UserResponse,
        r#"
        SELECT
            id,
            username,
            email
        FROM users
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn create_user(
    pool: &PgPool,
    request: &CreateUserRequest,
) -> Result<UserResponse, AppError> {
    let user_id = Uuid::now_v7();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(request.password_hash.as_bytes(), &salt)
        .map_err(|error| {
            eprintln!("Failed to hash password: {error}");

            AppError::Internal
        })?
        .to_string();

    let user = sqlx::query!(
        r#"
        INSERT INTO users (
            id,
            username,
            email,
            password_hash
        )
        VALUES ($1, $2, $3, $4)
        RETURNING id, username, email
        "#,
        user_id,
        request.username,
        request.email,
        password_hash
    )
    .fetch_one(pool)
    .await
    .map_err(|error| {
        eprintln!("Failed to create user: {error}");

        if let sqlx::Error::Database(db_error) = &error {
            if db_error.constraint() == Some("users_username_key")
                || db_error.constraint() == Some("users_email_key")
            {
                return AppError::DuplicateUser;
            }
        }

        AppError::Database
    })?;

    Ok(UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
    })
}
