use actix_web::{HttpResponse, web};
use sqlx::PgPool;

pub async fn health(pool: web::Data<PgPool>) -> HttpResponse {
    let result = sqlx::query("SELECT 1").execute(pool.get_ref()).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "ok",
            "database": "connected"
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "database": "disconnected"
        })),
    }
}
