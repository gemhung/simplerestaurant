#[derive(Default, Debug, sqlx::FromRow, serde::Serialize)]
pub struct Model {
    pub id: i64,
    pub status: String,
    pub name: String,
    pub cooking_time: i64,
    pub created_at: chrono::NaiveDateTime,
}
