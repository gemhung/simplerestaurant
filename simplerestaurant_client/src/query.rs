use crate::utils;
pub async fn query_table(addr: &str, table: i32) -> serde_json::Value {
    let response = utils::client()
        .get(format!("{}/items", addr))
        .query(&[("table", table.to_string())])
        .send()
        .await
        .expect("Failed to execute request.");
    let json: serde_json::Value = response.json().await.expect("Failed to get json body");

    json
}

pub async fn query_table_with_name(addr: &str, table: i32, name: String) -> serde_json::Value {
    let response = utils::client()
        .get(&format!("{}/items/{}", addr, name))
        .query(&[("table", table.to_string())])
        .send()
        .await
        .expect("Failed to execute request.");
    let json: serde_json::Value = response.json().await.expect("Failed to get json body");

    json
}
