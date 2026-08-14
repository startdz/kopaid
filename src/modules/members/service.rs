use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError::{self, Database},
    modules::members::dto::{CreateMemberRequest, MemberResponse},
};

pub async fn list_members(pool: &PgPool) -> Result<Vec<MemberResponse>, AppError> {
    let members = sqlx::query_as!(
        MemberResponse,
        r#"
            SELECT
                id,
                member_number,
                full_name,
                email,
                phone,
                address,
                joined_at
            FROM members
            ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|error| {
        eprintln!("Failed to list members: {error}");
        AppError::Database
    })?;

    Ok(members)
}

pub async fn create_member(
    pool: &PgPool,
    request: &CreateMemberRequest,
) -> Result<MemberResponse, AppError> {
    let id = Uuid::now_v7();
    let member = sqlx::query!(
        r#"
        INSERT INTO members (
            id,
            member_number,
            full_name,
            email,
            phone,
            address
        )
        VALUES (
            $1,
            'MBR-' || LPAD(
                nextval('member_number_seq')::text,
                6,
                '0'
            ),
            $2,
            $3,
            $4,
            $5
        )
        RETURNING
            id,
            member_number,
            full_name,
            email,
            phone,
            address,
            joined_at
        "#,
        id,
        request.full_name,
        request.email,
        request.phone,
        request.address,
    )
    .fetch_one(pool)
    .await
    .map_err(|error| {
        eprintln!("Failed to create member: {error}");
        Database
    })?;

    Ok(MemberResponse {
        id,
        member_number: member.member_number,
        full_name: member.full_name,
        email: member.email,
        phone: member.phone,
        address: member.address,
        joined_at: member.joined_at,
    })
}
