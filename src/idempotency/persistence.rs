use super::IdempotencyKey;
use crate::models::schema::idempotency::HeaderPairRecord;
use actix_web::body::to_bytes;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use sqlx::{Executor, PgPool};
use sqlx::{Postgres, Transaction};

pub async fn get_saved_response(
    pool: &PgPool,
    table: i32,
    idempotency_key: &IdempotencyKey,
) -> Result<Option<HttpResponse>, anyhow::Error> {
    let query = format!(
        "
            SELECT 
                idempotency_key,
                response_status_code,
                response_headers, 
                response_body,
                created_at
            FROM table{}.idempotency
            WHERE 
              idempotency_key = $1
        ",
        table
    );
    let ret = sqlx::query_as::<_, crate::models::schema::idempotency::Model>(&query)
        .bind(idempotency_key.as_ref())
        .fetch_optional(pool)
        .await?;
    if let Some(inner) = ret {
        let status_code = StatusCode::from_u16(inner.response_status_code.try_into()?)?;
        let mut response = HttpResponse::build(status_code);
        for HeaderPairRecord { name, value } in inner.response_headers {
            response.append_header((name, value));
        }
        Ok(Some(response.body(inner.response_body)))
    } else {
        Ok(None)
    }
}

pub async fn save_response(
    mut transaction: Transaction<'static, Postgres>,
    table: i32,
    idempotency_key: &IdempotencyKey,
    http_response: HttpResponse,
) -> Result<HttpResponse, anyhow::Error> {
    let (response_head, body) = http_response.into_parts();
    let body = to_bytes(body).await.map_err(|e| anyhow::anyhow!("{}", e))?;
    let status_code = response_head.status().as_u16() as i16;
    let headers = {
        let mut h = Vec::with_capacity(response_head.headers().len());
        for (name, value) in response_head.headers().iter() {
            let name = name.as_str().to_owned();
            let value = value.as_bytes().to_owned();
            h.push(HeaderPairRecord { name, value });
        }
        h
    };
    let update_sql = format!(
        "

                    UPDATE table{}.idempotency
                    SET 
                        response_status_code = $2, 
                        response_headers = $3,
                        response_body = $4 
                    WHERE
                        idempotency_key = $1
        ",
        table
    );
    transaction
        .execute(
            sqlx::query(&update_sql)
                .bind(idempotency_key.as_ref())
                .bind(status_code)
                .bind(headers)
                .bind(body.as_ref()),
        )
        .await?;
    transaction.commit().await?;

    let http_response = response_head.set_body(body).map_into_boxed_body();
    Ok(http_response)
}

#[allow(clippy::large_enum_variant)]
pub enum NextAction {
    // Return transaction for later usage
    StartProcessing(Transaction<'static, Postgres>),
    ReturnSavedResponse(HttpResponse),
}

pub async fn try_processing(
    pool: &PgPool,
    table: i32,
    idempotency_key: &IdempotencyKey,
) -> Result<NextAction, anyhow::Error> {
    let mut transaction = pool.begin().await?;
    let insert_sql = format!(
        "
        INSERT INTO table{}.idempotency (
            idempotency_key,
            created_at
        ) 
        VALUES ($1, now()) 
        ON CONFLICT DO NOTHING
        ",
        table
    );
    let query = sqlx::query(&insert_sql).bind(idempotency_key.as_ref());
    let n_inserted_rows = transaction.execute(query).await?.rows_affected();
    if n_inserted_rows > 0 {
        Ok(NextAction::StartProcessing(transaction))
    } else {
        drop(transaction);
        let saved_response = get_saved_response(pool, table, idempotency_key)
            .await?
            .ok_or_else(|| anyhow::anyhow!("We expected a saved response, we didn't find it"))?;
        Ok(NextAction::ReturnSavedResponse(saved_response))
    }
}
