use sqlx::FromRow;
use chrono::NaiveDate;

#[derive(FromRow)]
pub struct Transaction {
    pub id: i64,
    pub category_id: i64,
    pub amount: i64,
    pub description: Option<String>,
    pub date: NaiveDate,
}
