use crate::{
    errors::AppError::{self, Conflict, Database},
    modules::roles::dto::{CreateRoleRequest, RoleResponse},
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn list_roles(pool: &PgPool) -> Result<Vec<RoleResponse>, AppError> {
    let roles = sqlx::query_as!(
        RoleResponse,
        r#"
        SELECT
            id,
            name,
            description
        FROM roles
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|error| {
        eprintln!("Failed to get list roles: {error}");

        Database
    })?;

    Ok(roles)
}

pub async fn create_role(
    pool: &PgPool,
    request: &CreateRoleRequest,
) -> Result<RoleResponse, AppError> {
    let role_id = Uuid::now_v7();
    let role = sqlx::query!(
        r#"
        INSERT INTO roles (
            id,
            name,
            description
        )
        VALUES ($1, $2, $3)
        RETURNING id, name, description
        "#,
        role_id,
        request.name,
        request.description
    )
    .fetch_one(pool)
    .await
    .map_err(|error| {
        if let sqlx::Error::Database(db_error) = &error {
            if db_error.constraint() == Some("roles_name_key") {
                return Conflict;
            }
        }

        eprintln!("Failed to create role: {error}");
        Database
    })?;

    Ok(RoleResponse {
        id: role.id,
        name: role.name,
        description: role.description,
    })
}
