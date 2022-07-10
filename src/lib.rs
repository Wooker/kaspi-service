use std::sync::Arc;
use reqwest::Client;
use kaspi_rs::upload_result::{UploadStatus, UploadResult};

pub(crate) async fn send_to_kaspi(product: serde_json::Value, client: Arc<Client>) -> anyhow::Result<serde_json::Value> {
    let product_vec = serde_json::to_value([product]).expect("Could not create json");
    let response = client.post(
        "https://kaspi.kz/shop/api/products/import"
    ).header("Content-Type", "text/plain").body(product_vec.to_string()).send().await.expect("Could not send request");

    let response_json = serde_json::to_value(response.json::<UploadStatus>().await.expect("Could not read response json")).expect("Could not create json");
    println!("{}", response_json);

    let mut product = product_vec.as_array().expect("Could not create an array").first().expect("Array is empty").clone();
    product.as_object_mut().expect("Could not create an object")
        .insert(
            String::from("code"),
            response_json.get("code").expect("No such field").to_owned()
        ).expect("Could not add a field");
    product.as_object_mut().expect("Could not create an object")
        .insert(
            String::from("status"),
            response_json.get("status").expect("No such field").to_owned()
        ).expect("Could not add a field");

    Ok(product.to_owned())
}

pub(crate) async fn check_code(code: &str, client: Arc<Client>) -> serde_json::Value {
    let mut response = client.get(
        format!("https://kaspi.kz/shop/api/products/import?i={}", code)
    ).send().await.expect("Could not send request");

    let mut response_json = serde_json::to_value(
        response.json::<UploadStatus>().await.expect("Could not fetch response json")
    ).expect("Could not convert to json");

    // TODO: handle states:
    //  * FINISHED
    //  * UPLOADED
    //  * ABORTED (new field - description)
    if response_json["status"].ne("UPLOADED") {
        response = client.get(
            format!("https://kaspi.kz/shop/api/products/import/result?i={}", code)
        ).send().await.expect("Could not send request");

        response_json = serde_json::to_value(
            response.json::<UploadResult>().await.expect("Could not parse response text")
        ).expect("Could not convert to json");
    }

    response_json
}
