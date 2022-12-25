pub mod routes;
pub mod json_processing;
pub mod entities;
pub mod store;

use uuid::Uuid;
use std::sync::Arc;
use reqwest::Client;
use serde_json::json;
use lazy_static::lazy_static;
use crate::{
    store::Store,
    entities::{upload_result::*, product::{Product, Record}}
};

use anyhow::{Result, Error};

lazy_static!{
    pub static ref STORE: Store = Store::new();
}

pub(crate) async fn send_to_kaspi(product: serde_json::Value, client: Arc<Client>) -> Result<String, String> {
    // Kaspi requires an array of products
    // Create a vector for a product
    let product_vec = serde_json::to_value([product]).expect("Could not create Value");

    // Get the element from the array
    let product_json = product_vec
        .as_array().expect("Could not create an array")
        .first().expect("Array is empty")
        .clone();

    // Generate Uuid from product contents
    let id = Uuid::new_v5(
        &Uuid::NAMESPACE_URL,
        serde_json::to_string(&product_json)
        .unwrap()
        .as_bytes()
    );

    if let Some(product) = STORE.insert_product(id, serde_json::from_value::<Product>(product_json).unwrap()).await {
        return Err(format!("Duplicate product {}", product.sku()));
    } else {
        // Take response for uploading request
        let response = client
            .post("https://kaspi.kz/shop/api/products/import")
            .header("Content-Type", "text/plain")
            .body(product_vec.to_string())
            .send()
            .await.expect("Could not upload to kaspi");

        // Convert response to json value
        let response = serde_json::to_value(
            response.json::<UploadStatus>()
            .await.expect("Could not convert json to UploadStatus")
        ).expect("Could not create Value");

        let code_string = response["code"].as_str().unwrap().to_string();

        if let Some(code) = STORE.insert_upload(id, code_string.clone()).await {
            return Err(format!("Duplicate upload {}", code));
        } else {
            // Log the upload
            log::info!("{}", format!("Uploaded: {:?}", id));
            Ok(code_string)
        }
    }
}

// TODO: split in different functions for uploaded, finished products
pub(crate) async fn check_code(id: &Uuid, client: Arc<Client>) -> Result<serde_json::Value, String> {
    if let Some((code, status)) = STORE.get_status(id).await {
        if status == Status::UPLOADED {
            check_status(id, code, status, client).await
        } else {
            let value = json!({
                "id": id,
                "status": status,
            });

            Ok(value)
        }
    } else {
        Err(format!("ID: '{}' is not found", id))
    }
}

pub(crate) async fn check_status(id: &Uuid, code: String, status: Status, client: Arc<Client>) -> Result<serde_json::Value, String> {
    // Get reponse of checking request
    let response = client.get(
        format!("https://kaspi.kz/shop/api/products/import?i={}", code)
    )
    .send()
    .await.expect("Could not get result response");

    // Convert response to json value
    let upload_status = response.json::<UploadStatus>().await.expect("Could not parse json as UploadStatus");

    match upload_status.get_status() {
        Status::FINISHED => {
            STORE.archive(id, Status::FINISHED).await;
            check_result(id, code, Status::FINISHED, client).await
        }
        Status::ABORTED => {
            STORE.archive(id, Status::ABORTED).await;
            check_result(id, code, Status::ABORTED, client).await
        }
        _ => {
            let value = json!({
                "id": id,
                "status": status,
            });

            Ok(value)
        }
    }
}

pub(crate) async fn check_result(id: &Uuid, code: String, status: Status, client: Arc<Client>) -> Result<serde_json::Value, String> {
    // Get response for result request
    let response = client.get(
        format!("https://kaspi.kz/shop/api/products/import/result?i={}", code)
    )
    .send()
    .await.expect("Could not get result response");


    let result = response.json::<UploadResult>()
        .await.expect("Could not parse json as UploadResult");

    STORE.insert_result(id.to_owned(), result.clone()).await;

    // Convert response to json
    let response_json = serde_json::to_value(result).expect("Could not create Value");

    let value = json!({
        "id": id,
        "status": status,
        "result": response_json
    });

    Ok(value)
}

pub async fn spawn_save() {
    use serde_json::Value;
    use crate::json_processing::{save_json, FILE_NAME};

    actix_rt::spawn(async move {
        log::info!("Saving...");

        let mut records: Vec<Value> = Vec::new();

        let products = STORE.products().await;
        for (id, product) in products.iter() {
            let (code, status) = STORE.get_status(id).await.unwrap();
            let result = STORE.get_result(id).await;

            let record = json!({
                "id": id,
                "code": code,
                "status": status,
                "product": product,
                "result": result
            });

            records.push(record);
        }
        let json = serde_json::to_value(records).expect("Could not create Value");
        save_json(FILE_NAME, json).await.expect("Could not save json file");

        log::info!("Saved!");
    }).await.expect("Could not save record");
}
