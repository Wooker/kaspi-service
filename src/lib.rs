pub mod routes;
pub mod json_processing;
pub mod entities;

use uuid::Uuid;
use std::sync::Arc;
use reqwest::Client;
use serde_json::json;
use crate::entities::{
    upload_result::*,
    product::Record,
};

pub(crate) async fn send_to_kaspi(product: serde_json::Value, client: Arc<Client>) -> anyhow::Result<Record> {
    // Kaspi requires an array of products
    // Create a vector for a product
    let product_vec = serde_json::to_value([product]).expect("Could not create json");

    // Take response for uploading request
    let response = client
        .post("https://kaspi.kz/shop/api/products/import")
        .header("Content-Type", "text/plain")
        .body(product_vec.to_string())
        .send()
        .await.expect("Could not send request");

    // Convert response to json value
    let response = serde_json::to_value(
        response.json::<UploadStatus>()
        .await.expect("Could not read response json")
    ).expect("Could not create json");

    // Get the element from the array
    let product = product_vec
        .as_array().expect("Could not create an array")
        .first().expect("Array is empty")
        .clone();

    // Create a record json
    let record_json = json!(
        {
            "id": Uuid::new_v4().to_string(),
            "product": product,
            "code": response["code"],
            "status": response["status"]
        }
    );

    // Convert json to Record type
    let record: Record = serde_json::from_value(record_json)
        .expect("Could not create a record from json");

    Ok(record)
}

pub(crate) async fn check_code(id: &str, code: &str, client: Arc<Client>) -> serde_json::Value {
    // Get reponse of checking request
    let mut response = client.get(
        format!("https://kaspi.kz/shop/api/products/import?i={}", code)
    ).send().await.expect("Could not send request");

    // Convert response to json value
    let upload_status = response.json::<UploadStatus>()
        .await.expect("Could not fetch response json");

    let status = upload_status.get_status();
    let mut response_json = serde_json::to_value(upload_status)
        .expect("Could not create json");

    // If uploading is complete, get result and update the record
    if status.as_str() != "UPLOADED" {
        // Get response for result request
        response = client.get(
            format!("https://kaspi.kz/shop/api/products/import/result?i={}", code)
        ).send().await.expect("Could not send request");

        // Convert response to json
        response_json = serde_json::to_value(
            response.json::<UploadResult>()
            .await.expect("Could not fetch response json")
        ).expect("Could not create json");

        // update record
    }

    json!({
        "id": id,
        "code": code,
        "status": status,
        "response": response_json
    })
}
