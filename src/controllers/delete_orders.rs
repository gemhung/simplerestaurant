use crate::controllers::utils as controllers_utils;
use crate::idempotency::{save_response, try_processing, IdempotencyKey, NextAction};
use crate::utils::e400;
use crate::utils::e500;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;
use rand::Rng;
use sqlx::{Executor, Postgres, Transaction};
use std::fmt::Write;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct DeleteOrder {
    table: i32,
    item_id: i64,
    idempotency_key: String,
}

pub async fn delete_orders(
    web::Json(DeleteOrder {
        table,
        item_id,
        idempotency_key,
    }): web::Json<DeleteOrder>,
) -> impl actix_web::Responder {
    // Validation: table
    let table = controllers_utils::validate_table(table)?;
    let idempotency_key: IdempotencyKey = idempotency_key.try_into().map_err(e400)?;
    tracing::debug!("{item_id}, {table}, {idempotency_key:?}");
    // Logic
    let pool = crate::boot::database::db();
    let mut transaction = match try_processing(pool, table, &idempotency_key)
        .await
        .map_err(e500)?
    {
        NextAction::StartProcessing(t) => t,
        NextAction::ReturnSavedResponse(saved_response) => {
            return Result::<_, crate::errors::AppError>::Ok(saved_response);
        }
    };
    delete_ordered_item(&mut transaction, table, item_id).await?;
    let response = actix_web::HttpResponse::Ok().json("deleted");
    let response = save_response(transaction, table, &idempotency_key, response)
        .await
        .map_err(e500)?;

    Ok(response)
}

async fn delete_ordered_item(
    transaction: &mut Transaction<'_, Postgres>,
    table: i32,
    item_id: i64,
) -> Result<(), sqlx::Error> {
    let delete_sql = format!(
        "
            DELETE FROM table{}.ordered_items 
            WHERE id = $1
        ",
        table
    );
    let query = sqlx::query(&delete_sql).bind(item_id);
    transaction.execute(query).await?;

    Ok(())
}
