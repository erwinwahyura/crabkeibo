use sqlx::FromRow;

#[derive(FromRow)]
pub struct Category {
    pub id: i64,
    pub name: String,
}
