use crate::utils;

pub async fn cancel_order(addr: &str, table: i32, item_id: i64) -> serde_json::Value {
    let body = serde_json::json!({
        "item_id": item_id,
        "table": table,
        // We expect the idempotency key as part of the
        // form data, not as an header
        "idempotency_key": uuid::Uuid::new_v4().to_string()
    });
    let response = utils::client()
        .delete(&format!("{}/items", addr))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");
    //let json: serde_json::Value = response.json().await.unwrap();
    let mut ret = vec![];
    match response.status() {
        reqwest::StatusCode::OK => {
            ret.push(format!("item {} is deleted in table {}", item_id, table));
        }

        _ => {
            let body: serde_json::Value =
                response.json().await.expect("Failed to get body as json");
            ret.push(format!("error: {}", body));
        }
    }
    let json = serde_json::to_value(ret).expect("Failed to convert to json");

    json
}
