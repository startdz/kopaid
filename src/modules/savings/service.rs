use bigdecimal::BigDecimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError::{self},
    modules::{
        savings::dto::{CreateSavingsRequest, SavingsResponse},
        savings_transactions::dto::{
            CreateSavingsTransactionRequest, SavingsTransactionResponse, SavingsTransactionType,
        },
    },
};

pub async fn list_savings(pool: &PgPool) -> Result<Vec<SavingsResponse>, AppError> {
    let savings = sqlx::query_as!(
        SavingsResponse,
        r#"
            SELECT
                id,
                account_number,
                member_id,
                product_id,
                balance,
                is_active,
                opened_at,
                created_at,
                updated_at
            FROM savings
            ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|error| {
        eprintln!("Failed to get all savings {error}");
        AppError::Database
    })?;

    Ok(savings)
}

pub async fn create_savings(
    pool: &PgPool,
    request: &CreateSavingsRequest,
) -> Result<SavingsResponse, AppError> {
    let member_exists = sqlx::query_scalar!(
        r#"
            SELECT EXISTS (
                SELECT 1
                FROM members
                WHERE id = $1
            ) AS "exists!"
        "#,
        request.member_id
    )
    .fetch_one(pool)
    .await
    .map_err(|error| {
        eprintln!("Failed to check member: {}", error);
        AppError::Database
    })?;

    if !member_exists {
        return Err(AppError::NotFound);
    }

    let product_exists = sqlx::query_scalar!(
        r#"
            SELECT EXISTS (
                SELECT 1
                FROM savings_products
                WHERE id = $1
                    AND is_active = TRUE
            ) as "exists!"
        "#,
        request.product_id
    )
    .fetch_one(pool)
    .await
    .map_err(|error| {
        eprintln!("Failed to check savings product: {}", error);
        AppError::Database
    })?;

    if !product_exists {
        return Err(AppError::NotFound);
    }

    let id = Uuid::now_v7();
    let result = sqlx::query!(
        r#"
            INSERT INTO savings (
                id,
                account_number,
                member_id,
                product_id
            )
            VALUES (
                $1,
                'SIMP-' || LPAD(
                nextval('savings_account_number_seq')::text,
                6,
                '0'
                ),
                $2,
                $3
            )
            RETURNING
                id,
                account_number,
                member_id,
                product_id,
                balance,
                is_active,
                opened_at,
                created_at,
                updated_at
        "#,
        id,
        request.member_id,
        request.product_id,
    )
    .fetch_one(pool)
    .await
    .map_err(|error| {
        if let sqlx::Error::Database(db_error) = &error {
            if db_error.constraint() == Some("uq_savings_member_product") {
                return AppError::Conflict;
            }
        }

        eprintln!("Failed to create savings account: {error}");
        AppError::Database
    })?;

    Ok(SavingsResponse {
        id,
        account_number: result.account_number,
        member_id: result.member_id,
        product_id: result.product_id,
        balance: result.balance,
        is_active: result.is_active,
        opened_at: result.opened_at,
        created_at: result.created_at,
        updated_at: result.updated_at,
    })
}

pub async fn create_savings_transaction(
    pool: &PgPool,
    current_user_id: Uuid,
    request: &CreateSavingsTransactionRequest,
) -> Result<SavingsTransactionResponse, AppError> {
    let mut tx = pool.begin().await.map_err(|error| {
        eprintln!("Failed to begin transaction: {}", error);
        AppError::Database
    })?;

    let savings = sqlx::query!(
        r#"
            SELECT
                id,
                balance,
                is_active
            FROM savings
            WHERE id = $1
            FOR UPDATE
        "#,
        request.savings_id
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|error| {
        eprintln!("Failed to find savings account: {}", error);
        AppError::Database
    })?;

    let Some(savings) = savings else {
        return Err(AppError::NotFound);
    };

    if !savings.is_active {
        return Err(AppError::Conflict);
    }

    if request.amount <= BigDecimal::from(0) {
        return Err(AppError::InvalidAmount);
    }

    let new_balance = match request.transaction_type {
        SavingsTransactionType::Deposit => &savings.balance + &request.amount,

        SavingsTransactionType::Withdrawal => {
            if savings.balance < request.amount {
                return Err(AppError::InsufficientBalance);
            }

            &savings.balance - &request.amount
        }
    };

    let transaction_id = Uuid::now_v7();

    let transaction_type = match request.transaction_type {
        SavingsTransactionType::Deposit => "deposit",
        SavingsTransactionType::Withdrawal => "withdrawal",
    };

    let transaction = sqlx::query!(
        r#"
            INSERT INTO savings_transactions (
                id,
                savings_id,
                transaction_type,
                amount,
                description,
                created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING
                id,
                savings_id,
                transaction_type,
                amount,
                reference_number,
                description,
                created_by,
                created_at
        "#,
        transaction_id,
        request.savings_id,
        transaction_type,
        request.amount,
        request.description,
        current_user_id,
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|error| {
        eprintln!("Failed to create savings transaction: {error}");
        AppError::Database
    })?;

    sqlx::query!(
        r#"
            UPDATE savings
            SET
                balance = $1,
                updated_at = NOW()
            WHERE id = $2
        "#,
        new_balance,
        request.savings_id,
    )
    .execute(&mut *tx)
    .await
    .map_err(|error| {
        eprintln!("Failed to update savings balance: {error}");
        AppError::Database
    })?;

    tx.commit().await.map_err(|error| {
        eprintln!("Failed to commit savings transaction: {error}");
        AppError::Database
    })?;

    Ok(SavingsTransactionResponse {
        id: transaction_id,
        savings_id: savings.id,
        transaction_type: transaction.transaction_type,
        amount: transaction.amount,
        reference_number: transaction.reference_number,
        description: transaction.description,
        created_by: transaction.created_by,
        created_at: transaction.created_at,
    })
}
