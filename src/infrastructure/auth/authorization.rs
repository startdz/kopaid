use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError::{self, Database, Forbidden};

pub async fn require_permission(
    pool: &PgPool,
    user_id: Uuid,
    permission: &str,
) -> Result<(), AppError> {
    let allowed = sqlx::query_scalar!(
        r#"
            SELECT EXISTS (
                SELECT 1
                FROM users u
                INNER JOIN roles r
                    ON r.id = u.role_id
                INNER JOIN role_permissions rp
                    ON rp.role_id = r.id
                INNER JOIN permissions p
                    ON p.id = rp.permission_id
                WHERE u.id = $1
                    AND p.name = $2
            ) AS "allowed!"
        "#,
        user_id,
        permission
    )
    .fetch_one(pool)
    .await
    .map_err(|error| {
        eprintln!("Failed to check permissions: {error}");
        Database
    })?;

    if !allowed {
        return Err(Forbidden);
    }

    Ok(())
}
