use crate::controllers::utils as controllers_utils;
use crate::idempotency::{save_response, try_processing, IdempotencyKey, NextAction};
use crate::utils::e400;
use crate::utils::e500;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;
use rand::Rng;
use sqlx::PgPool;
use sqlx::{Executor, Postgres, Transaction};
use std::fmt::Write;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreateOrder {
    pub item_name: String,
    pub table: i32,
    pub idempotency_key: String,
}

pub async fn create_orders(
    web::Json(CreateOrder {
        item_name,
        idempotency_key,
        table,
    }): web::Json<CreateOrder>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, crate::errors::AppError> {
    // Validation
    let table = controllers_utils::validate_table(table)?;
    let idempotency_key: IdempotencyKey = idempotency_key.try_into().map_err(e400)?;
    tracing::debug!("{item_name}, {table}, {idempotency_key:?}");
    // Logic
    let mut transaction = match try_processing(&pool, table, &idempotency_key)
        .await
        .map_err(e500)?
    {
        NextAction::StartProcessing(t) => t,
        NextAction::ReturnSavedResponse(saved_response) => {
            return Result::<_, crate::errors::AppError>::Ok(saved_response);
        }
    };
    let cooking_time = rand::thread_rng().gen_range(300..900); // 5 min - 15 mins
    insert_ordered_item(&mut transaction, table, "added", &item_name, cooking_time).await?;
    let response = actix_web::HttpResponse::Ok().json("added");
    let response = save_response(transaction, table, &idempotency_key, response)
        .await
        .map_err(e500)?;

    Ok(response)
}

async fn insert_ordered_item(
    transaction: &mut Transaction<'_, Postgres>,
    table: i32,
    status: &str,
    name: &str,
    cooking_time: i64,
) -> Result<(), sqlx::Error> {
    let now = chrono::offset::Local::now();
    let insert_sql = format!(
        "
            INSERT INTO table{}.ordered_items (
                status, 
                name, 
                cooking_time, 
                created_at
            )
            VALUES ($1,$2,$3,$4)
        ",
        table
    );
    let query = sqlx::query(&insert_sql)
        .bind(status)
        .bind(name)
        .bind(cooking_time)
        .bind(now);

    transaction.execute(query).await?;

    Ok(())
}
