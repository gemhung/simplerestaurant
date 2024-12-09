use crate::utils;

pub async fn send_create_orders(addr: &str, orders: Vec<(&str, &str)>) -> serde_json::Value {
    let client = std::sync::Arc::new(utils::client());
    let mut handles = vec![];
    for (name, table) in orders.iter() {
        let body = serde_json::json!({
            "item_name": name,
            "table": table.parse::<i32>().unwrap(),
            // We expect the idempotency key as part of the
            // form data, not as an header
            "idempotency_key": uuid::Uuid::new_v4().to_string()
        });
        let client = client.clone();
        // Concurrently executing tasks
        let addr = addr.to_string();
        let handle = tokio::task::spawn(async move {
            let response = client
                .post(&format!("{}/items", addr))
                .json(&body)
                .send()
                .await
                .expect("Failed to execute request.");
            response
        });
        handles.push(handle);
    }

    let mut ret = vec![];
    for (index, handle) in handles.into_iter().enumerate() {
        let response = handle.await.unwrap();
        match response.status() {
            reqwest::StatusCode::OK => {
                ret.push(format!(
                    "{} is added in table {}",
                    orders[index].0, orders[index].1
                ));
            }

            _ => {
                let body: serde_json::Value =
                    response.json().await.expect("Failed to get body as json");
                ret.push(format!("error: {}", body.to_string()));
            }
        }
    }
    let json = serde_json::to_value(ret).expect("Failed to convert to json");

    json
}
