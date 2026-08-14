use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    modules::savings_products::dto::{CreateSavingsProductRequest, SavingsProductResponse},
};

pub async fn list_savings_products(pool: &PgPool) -> Result<Vec<SavingsProductResponse>, AppError> {
    let products = sqlx::query_as!(
        SavingsProductResponse,
        r#"
            SELECT
                id,
                code,
                name,
                description,
                min_amount,
                max_amount,
                is_active,
                created_at,
                updated_at
            FROM savings_products
            ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|error| {
        eprintln!("Failed to get all products savings {error}");
        AppError::Database
    })?;

    Ok(products)
}

pub async fn create_savings_product(
    pool: &PgPool,
    request: &CreateSavingsProductRequest,
) -> Result<SavingsProductResponse, AppError> {
    let id = Uuid::now_v7();

    let product = sqlx::query!(
        r#"
        INSERT INTO savings_products (
            id,
            code,
            name,
            description,
            min_amount,
            max_amount
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING
            id,
            code,
            name,
            description,
            min_amount,
            max_amount,
            is_active,
            created_at,
            updated_at
        "#,
        id,
        request.code,
        request.name,
        request.description,
        request.min_amount,
        request.max_amount
    )
    .fetch_one(pool)
    .await
    .map_err(|error| {
        eprintln!("Failed to create savings product: {error}");
        AppError::Database
    })?;

    Ok(SavingsProductResponse {
        id,
        code: product.code,
        name: product.name,
        description: product.description,
        min_amount: product.min_amount,
        max_amount: product.max_amount,
        is_active: product.is_active,
        created_at: product.created_at,
        updated_at: product.updated_at,
    })
}
