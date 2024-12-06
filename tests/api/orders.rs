use crate::helpers::spawn_app;
use rand::Rng;

#[tokio::test]
async fn create_an_order() {
    // Arrange
    let app = spawn_app().await;

    let table = rand::thread_rng().gen_range(1..=100);
    let request_body = serde_json::json!({
        "item_name": "rice",
        "table": table,
        // We expect the idempotency key as part of the
        // form data, not as an header
        "idempotency_key": uuid::Uuid::new_v4().to_string()
    });
    let response: serde_json::Value = app
        .post_create_orders(&request_body)
        .await
        .json()
        .await
        .unwrap();
    assert_eq!(response.as_str().unwrap(), "added");
}

#[tokio::test]
async fn delete_an_order() {
    // Arrange
    let app = spawn_app().await;
    let table = rand::thread_rng().gen_range(1..=100);

    // 1. First create an order
    let create_request_body = serde_json::json!({
        "item_name": "rice",
        "table": table,
        // We expect the idempotency key as part of the
        // form data, not as an header
        "idempotency_key": uuid::Uuid::new_v4().to_string()
    });
    let response: serde_json::Value = app
        .post_create_orders(&create_request_body)
        .await
        .json()
        .await
        .unwrap();
    assert_eq!(response.as_str().unwrap(), "added");

    // 2. Get all items and expected to have one item only
    let response: Vec<serde_json::Value> = app.get_all_items(table).await.json().await.unwrap();
    assert!(response.len() == 1);
    let item_id = response[0]["id"].as_i64().unwrap();

    // 3. Delete the item which was just added
    let request_body = serde_json::json!({
        "item_id": item_id,
        "table": table,
        // We expect the idempotency key as part of the
        // form data, not as an header
        "idempotency_key": uuid::Uuid::new_v4().to_string()
    });
    let response: serde_json::Value = app.delete_orders(&request_body).await.json().await.unwrap();
    assert_eq!(response.as_str().unwrap(), "deleted");
}

#[tokio::test]
async fn get_all_items() {
    let app = spawn_app().await;
    let table = rand::thread_rng().gen_range(1..=100);
    let response: Vec<serde_json::Value> = app.get_all_items(table).await.json().await.unwrap();
    assert!(response.len() == 0);
}

#[tokio::test]
async fn get_specified_items() {
    let app = spawn_app().await;
    let table = rand::thread_rng().gen_range(1..=100);
    let name = "rice";
    // 1. Get specified items where name is "rice" and expected to get zero items
    let response: Vec<serde_json::Value> = app
        .get_specified_items(table, name)
        .await
        .json()
        .await
        .unwrap();
    assert!(response.len() == 0);

    // 2. Add an order where name is "rice"
    let request_body = create_order_body(table, "rice");
    let response: serde_json::Value = app
        .post_create_orders(&request_body)
        .await
        .json()
        .await
        .unwrap();
    assert_eq!(response.as_str().unwrap(), "added");

    // 3. Add an order where name is "noodle"
    let request_body = create_order_body(table, "noodle");
    let response: serde_json::Value = app
        .post_create_orders(&request_body)
        .await
        .json()
        .await
        .unwrap();
    assert_eq!(response.as_str().unwrap(), "added");

    // 4. Add an order where name is "bread"
    let request_body = create_order_body(table, "bread");
    let response: serde_json::Value = app
        .post_create_orders(&request_body)
        .await
        .json()
        .await
        .unwrap();
    assert_eq!(response.as_str().unwrap(), "added");

    // 5. Add an order where name is "rice"
    let request_body = create_order_body(table, "rice");
    let response: serde_json::Value = app
        .post_create_orders(&request_body)
        .await
        .json()
        .await
        .unwrap();
    assert_eq!(response.as_str().unwrap(), "added");

    // 6. Get specified items where name is "rice" and expected to get 2 items
    let response: Vec<serde_json::Value> = app
        .get_specified_items(table, "rice")
        .await
        .json()
        .await
        .unwrap();
    assert!(response.len() == 2);
}

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

fn create_order_body(table: i32, name: &str) -> serde_json::Value {
    let request_body = serde_json::json!({
        "item_name": name,
        "table": table,
        // We expect the idempotency key as part of the
        // form data, not as an header
        "idempotency_key": uuid::Uuid::new_v4().to_string()
    });

    request_body
}
