use crate::controllers::utils as controllers_utils;
use crate::idempotency::{save_response, try_processing, IdempotencyKey, NextAction};
use crate::utils::e400;
use crate::utils::e500;
use actix_web::web;
use sqlx::PgPool;
use sqlx::{Executor, Postgres, Transaction};

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
    pool: web::Data<PgPool>,
) -> impl actix_web::Responder {
    // Validation: table
    let table = controllers_utils::validate_table(table)?;
    let idempotency_key: IdempotencyKey = idempotency_key.try_into().map_err(e400)?;
    tracing::debug!("{item_id}, {table}, {idempotency_key:?}");
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
    let row_affected = delete_ordered_item(&mut transaction, table, item_id).await?;
    if row_affected == 0 {
        return Err(crate::errors::AppError::no_such_item(format!(
            "No such item id({})",
            item_id
        )));
    }
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
) -> Result<u64, sqlx::Error> {
    let delete_sql = format!(
        "
            DELETE FROM table{}.ordered_items 
            WHERE id = $1
        ",
        table
    );
    let query = sqlx::query(&delete_sql).bind(item_id);
    let rows_affected = transaction.execute(query).await?.rows_affected();

    Ok(rows_affected)
}
