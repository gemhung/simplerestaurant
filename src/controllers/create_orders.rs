use crate::idempotency::{save_response, try_processing, IdempotencyKey, NextAction};
use crate::utils::e400;
use crate::utils::e500;
use actix_web::web;

/*
    {
      "item_name": "foo",
      “qty”: 2,
      "table": 1,
      "created_at": "TIMESTAMP"
      “serving_staff”: “John”
      “order_id”: “uuid1”
    },

*/

#[derive(serde::Deserialize)]
pub struct CreateOrder {
    item_name: String,
    table: i32,
    idempotency_key: String,
}

pub async fn create_orders(
    web::Json(CreateOrder {
        item_name,
        table,
        idempotency_key,
    }): web::Json<CreateOrder>, //) -> Result<HttpResponse, actix_web::Error> {
) -> impl actix_web::Responder {
    //let user_id = user_id.into_inner();
    let pool = crate::boot::database::db();
    let idempotency_key: IdempotencyKey = idempotency_key.try_into().map_err(e400)?;
    let order_id = uuid::Uuid::new_v4();
    let mut transaction = match try_processing(&pool, &idempotency_key, &order_id)
        .await
        .map_err(e500)?
    {
        NextAction::StartProcessing(t) => t,
        NextAction::ReturnSavedResponse(saved_response) => {
            //success_message().send();
            return Result::<_, crate::errors::AppError>::Ok(saved_response);
        }
    };
    //let issue_id = insert_newsletter_issue(&mut transaction, &title, &text_content, &html_content)
    //    .await
    //    .context("Failed to store newsletter issue details")
    //    .map_err(e500)?;
    //enqueue_delivery_tasks(&mut transaction, issue_id)
    //    .await
    //    .context("Failed to enqueue delivery tasks")
    //    .map_err(e500)?;

    let response = actix_web::HttpResponse::Ok().finish();
    let response = save_response(transaction, &idempotency_key, order_id, response)
        .await
        .map_err(e500)?;
    //success_message().send();
    Ok(response)
}
