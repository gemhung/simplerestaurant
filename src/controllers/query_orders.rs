use crate::errors::AppError;
use crate::models::OrderedItemsModel;
use actix_web::web;
use sqlx::{Executor, PgPool};
use crate::controllers::utils as controllers_utils;

#[derive(serde::Deserialize)]
pub struct Filter {
    pub table: Option<i32>,
    pub name: Option<String>,
}

pub async fn get_all_ordered_items(filter: web::Query<Filter>) -> impl actix_web::Responder {
    // Validation: table
    let table = controllers_utils::validate_table(filter.table)?;
    // Logic
    let pool = crate::boot::database::db();
    let select_sql = format!(
        r#"
            SELECT *
            FROM table{}.ordered_items
            WHERE status = 'added'
        "#,
        table
    );
    let remain_orders = sqlx::query_as::<_, OrderedItemsModel>(&select_sql)
        .fetch_all(pool)
        .await?;

    let values = serde_json::to_value(remain_orders)?;

    Result::<_, AppError>::Ok(actix_web::HttpResponse::Ok().json(values))
}

pub async fn get_specified_ordered_items(
    name: web::Path<String>,
    filter: actix_web::web::Query<Filter>,
) -> impl actix_web::Responder {
    // Validation: table
    let table = controllers_utils::validate_table(filter.table)?;
    // Logic
    let pool = crate::boot::database::db();
    let select_sql = format!(
        r#"
            SELECT *
            FROM table{}.ordered_items
            WHERE status = 'added' and name = $1
        "#,
        table
    );
    let remain_orders = sqlx::query_as::<_, OrderedItemsModel>(&select_sql)
        .bind(&*name)
        .fetch_all(pool)
        .await?;
    let values = serde_json::to_value(remain_orders)?;

    Result::<_, AppError>::Ok(actix_web::HttpResponse::Ok().json(values))
}

