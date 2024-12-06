use crate::helpers::spawn_app;
use rand::Rng;

#[tokio::test]
async fn order_creation_is_idempotent() {
    // Arrange
    let app = spawn_app().await;

    let table = rand::thread_rng().gen_range(1..=100);
    // Act - Part 1 - create an order
    let request_body = serde_json::json!({
        "item_name": "rice",
        "table": table,
        // We expect the idempotency key as part of the
        // form data, not as an header
        "idempotency_key": uuid::Uuid::new_v4().to_string()
    });
    let request_body = serde_json::to_value(request_body).unwrap();
    let response: serde_json::Value = app
        .post_create_orders(&request_body)
        .await
        .json()
        .await
        .unwrap();
    assert_eq!(response.as_str().unwrap(), "added");

    // Act - Part 2 -
    let response: Vec<serde_json::Value> = app.get_all_items(table).await.json().await.unwrap();
    assert!(response.len() == 1);

    // Act - Part 3 - create an order **again**
    let response: serde_json::Value = app
        .post_create_orders(&request_body)
        .await
        .json()
        .await
        .unwrap();
    assert_eq!(response.as_str().unwrap(), "added");

    // Act - Part 4 - check if there is still one item only
    let response: Vec<serde_json::Value> = app.get_all_items(table).await.json().await.unwrap();
    assert!(response.len() == 1);
}

#[tokio::test]
async fn concurrent_create_orders() {
    // Arrange
    let app = spawn_app().await;

    // Act - Submit two newsletter forms concurrently
    let table = rand::thread_rng().gen_range(1..=100);
    // Act - Part 1 - create an order
    let request_body = serde_json::json!({
        "item_name": "rice",
        "table": table,
        // We expect the idempotency key as part of the
        // form data, not as an header
        "idempotency_key": uuid::Uuid::new_v4().to_string()
    });
    let response1 = app.post_create_orders(&request_body);
    let response2 = app.post_create_orders(&request_body);
    let (response1, response2) = tokio::join!(response1, response2);

    assert_eq!(response1.status(), response2.status());
    assert_eq!(
        response1.text().await.unwrap(),
        response2.text().await.unwrap()
    );
}
