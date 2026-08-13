use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError::{self, Conflict, Database},
    modules::permissions::dto::{CreatePermissionRequest, PermissionResponse},
};

pub async fn list_permissions(pool: &PgPool) -> Result<Vec<PermissionResponse>, AppError> {
    let permissions = sqlx::query_as!(
        PermissionResponse,
        r#"
            SELECT id, name, description
            FROM permissions
            ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|error| {
        eprintln!("Failed to get list roles: {error}");

        Database
    })?;

    Ok(permissions)
}

pub async fn create_permission(
    pool: &PgPool,
    request: &CreatePermissionRequest,
) -> Result<PermissionResponse, AppError> {
    let permission_id = Uuid::now_v7();

    let permission = sqlx::query!(
        r#"
            INSERT INTO permissions (id, name, description)
            VALUES ($1, $2, $3)
            RETURNING id, name, description
        "#,
        permission_id,
        request.name,
        request.description
    )
    .fetch_one(pool)
    .await
    .map_err(|error| {
        if let sqlx::Error::Database(db_error) = &error {
            if db_error.constraint() == Some("permissions_name_key") {
                return Conflict;
            }
        }
        eprintln!("Failed to create permission: {error}");
        Database
    })?;

    Ok(PermissionResponse {
        id: permission_id,
        name: permission.name,
        description: permission.description,
    })
}
