#[derive(
    //Default, Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize,
    Default, Debug, sqlx::FromRow
)]
pub struct Model {
    pub id: i64,
    pub order_id: String,
    pub status: String,
    pub name: String,
    pub cooking_time: i64,
    pub created_at: chrono::NaiveDateTime,
}

