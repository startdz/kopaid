use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;

use crate::errors::AppError;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}

pub async fn run_seeds(pool: &PgPool) -> Result<(), AppError> {
    seed_roles(pool).await?;
    seed_permissions(pool).await?;
    seed_role_permissions(pool).await?;

    Ok(())
}

async fn seed_roles(pool: &PgPool) -> Result<(), AppError> {
    sqlx::query!(
        r#"
            INSERT INTO roles (id, name, description)
            VALUES ($1, $2, $3)
            ON CONFLICT (name) DO NOTHING 
        "#,
        Uuid::now_v7(),
        "SUPER_ADMIN",
        "System Administrator KOPAID"
    )
    .execute(pool)
    .await
    .map_err(|error| {
        eprintln!("Failed to seed Administrator role: {error}");
        AppError::Database
    })?;

    Ok(())
}

async fn seed_permissions(pool: &PgPool) -> Result<(), AppError> {
    let permissions = [
        ("user:read", "Read users"),
        ("user:create", "Create users"),
        ("user:update", "Update users"),
        ("user:delete", "Delete users"),
        ("user:assigned", "Assigned user to role"),
        ("role:read", "Read roles"),
        ("role:create", "Create roles"),
        ("role:update", "Update roles"),
        ("role:delete", "Delete roles"),
        ("role:assign-permission", "Assign permission to role"),
        ("permission:read", "Read permissions"),
        ("permission:create", "Create permissions"),
        ("permission:update", "Update permissions"),
        ("permission:delete", "Delete permissions"),
    ];

    for (name, description) in permissions {
        sqlx::query!(
            r#"
            INSERT INTO permissions (
                id,
                name,
                description
            )
            VALUES ($1, $2, $3)
            ON CONFLICT (name) DO NOTHING
            "#,
            Uuid::now_v7(),
            name,
            description,
        )
        .execute(pool)
        .await
        .map_err(|error| {
            eprintln!("Failed to seed permission {name}: {error}");
            AppError::Database
        })?;
    }

    Ok(())
}

async fn seed_role_permissions(pool: &PgPool) -> Result<(), AppError> {
    let role = sqlx::query!(
        r#"
        SELECT id
        FROM roles
        WHERE name = 'SUPER_ADMIN'
        "#
    )
    .fetch_one(pool)
    .await
    .map_err(|error| {
        eprintln!("Failed to find SUPER_ADMIN role: {error}");
        AppError::Database
    })?;

    let permissions = sqlx::query!(
        r#"
        SELECT id
        FROM permissions
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|error| {
        eprintln!("Failed to fetch permissions: {error}");
        AppError::Database
    })?;

    for permission in permissions {
        sqlx::query!(
            r#"
            INSERT INTO role_permissions (
                role_id,
                permission_id
            )
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#,
            role.id,
            permission.id,
        )
        .execute(pool)
        .await
        .map_err(|error| {
            eprintln!("Failed to assign permission: {error}");
            AppError::Database
        })?;
    }

    Ok(())
}
