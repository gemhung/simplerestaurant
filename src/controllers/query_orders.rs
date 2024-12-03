use crate::models::OrderedItemsModel;
use sqlx::{Executor, PgPool};

pub async fn get_all_ordered_items() -> impl actix_web::Responder {
    let pool = crate::boot::database::db();
    let remain_orders = sqlx::query_as::<_, OrderedItemsModel>(
        r#"
        SELECT *
        FROM ordered_items
        WHERE status = "added"
        "#,
    )
    .fetch_all(pool)
    .await?;

    Result::<_, crate::errors::AppError>::Ok("".to_string())
}

pub async fn get_one_ordered_item() -> impl actix_web::Responder {
    let pool = crate::boot::database::db();
    let name = "".to_string();
    let remain_orders = sqlx::query_as::<_, OrderedItemsModel>(
        r#"
        SELECT *
        FROM ordered_items
        WHERE status = "added" and name = $1
        "#,
    )
    .bind(name)
    .fetch_all(pool)
    .await?;

    Result::<_, crate::errors::AppError>::Ok("")
}
