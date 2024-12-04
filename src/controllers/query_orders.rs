use crate::errors::AppError;
use crate::models::OrderedItemsModel;
use actix_web::web;
use sqlx::{Executor, PgPool};

#[derive(serde::Deserialize)]
pub struct Filter {
    pub table: Option<i32>,
    pub name: Option<String>,
}

pub async fn get_all_ordered_items(filter: web::Query<Filter>) -> impl actix_web::Responder {
    let table = validate_table(filter.table)?;
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
    let table = validate_table(filter.table)?;
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

fn validate_table(table: Option<i32>) -> Result<i32, AppError> {
    let table = match table {
        Some(table @ 1..=100) => table,
        Some(_) => {
            return Err(AppError::invalid_filter(
                "Expecting table to be in a range from 1 to 100",
            ));
        }
        None => {
            return Err(AppError::missing_filter("Missing table filter"));
        }
    };

    Ok(table)
}
