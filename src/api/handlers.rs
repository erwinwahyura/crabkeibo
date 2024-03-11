use axum::{
    http::StatusCode,
    Json, Extension,
};
use sqlx::SqlitePool;
use crate::services::{self, *};
use crate::models::{Transaction};

// Example handler for creating a transaction
pub async fn create_transaction(Extension(pool): Extension<SqlitePool>, Json(payload): Json<CreateTransactionRequest>) -> Result<StatusCode, StatusCode> {
    let result = services::create_transaction(&pool, payload.category_id, payload.amount, payload.description, payload.date).await;
    match result {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Handler to list all transactions
pub async fn list_transactions(Extension(pool): Extension<PgPool>) -> Result<Json<Vec<Transaction>>, StatusCode> {
    match services::list_transactions(&pool).await {
        Ok(transactions) => Ok(Json(transactions)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Define other handlers (e.g., update_transaction, delete_transaction, get_transaction, list_transactions) in a similar way
