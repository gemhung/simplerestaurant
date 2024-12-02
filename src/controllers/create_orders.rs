
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
pub struct CreateOrders {
    item_name: String,
    qty: i32,
    table: i32,
}


pub async fn create_orders(
) -> impl actix_web::Responder {



    ""
}


