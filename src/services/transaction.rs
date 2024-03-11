use crate::models::Transaction;
use sqlx::{Pool, Sqlite, query};
use chrono::NaiveDate;

pub async fn create_transaction(pool: &PgPool, category_id: i32, amount: f64, description: Option<String>, date: NaiveDate) -> Result<Transaction, sqlx::Error> {
    let transaction = sqlx::query_as!(
        Transaction,
        "INSERT INTO transactions (category_id, amount, description, date) VALUES ($1, $2, $3, $4) RETURNING id, category_id, amount, description, date",
        category_id, amount, description, date
    )
    .fetch_one(pool)
    .await?;

    Ok(transaction)
}

pub async fn list_transactions(pool: &PgPool) -> Result<Vec<Transaction>, sqlx::Error> {
    let transactions = sqlx::query_as!(
        Transaction,
        "SELECT id, category_id, amount, description, date FROM transactions"
    )
    .fetch_all(pool)
    .await?;

    Ok(transactions)
}
