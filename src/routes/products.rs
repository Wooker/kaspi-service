use actix_web::{get, post, web, HttpResponse, Responder, HttpResponseBuilder, http::StatusCode, delete};
use reqwest::Client;
use serde_json::{json, Value};
use futures::future;
use crate::{
    STORE,
    send_to_kaspi,
    entities::{product::Product, upload_result::Status},
};

#[get("/")]
async fn show_all() -> impl Responder {
    let mut json: Vec<Value> = Vec::new();

    for (id, product) in STORE.products().await.iter() {
        let (code, status) = STORE.get_status(id).await.unwrap();

        let entry = json!({
            "id": id,
            "sku": product.sku(),
            "code": code,
            "status": status
        });

        json.push(entry);
    }

    HttpResponse::Ok().json(json)
}

#[get("/{id}")]
async fn show(path: web::Path<String>) -> impl Responder {
    let id = uuid::Uuid::parse_str(&path.into_inner()).unwrap();

    if let Some((code, status)) = STORE.get_status(&id).await {
        let product = STORE.get_product(&id).await.unwrap();
        let result = STORE.get_result(&id).await;

        let json = json!({
            "id": id,
            "code": code,
            "status": status,
            "product": product,
            "result": result
        });

        HttpResponse::Ok().json(json)
    } else {
        HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish()
    }
}

#[post("/")]
async fn add(products: web::Json<Vec<Product>>, client: web::Data<Client>) -> impl Responder {
    let results = future::join_all(
        products.into_inner().iter().map(|product| {
            let product_json = serde_json::to_value(
                product
                .clone()
            ).expect("Could not convert to json");

            send_to_kaspi(
                product_json, client.clone().into_inner()
            )
        }
    )).await;

    let mut duplicates: Vec<String> = Vec::new();
    let mut codes: Vec<String> = Vec::new();
    for result in results.into_iter() {
        let _ = result.as_ref()
            .map_err(|s| duplicates.push(s.to_owned()))
            .map(|s| codes.push(s.to_owned()));
    }

    if duplicates.is_empty() {
        let json = serde_json::to_value(codes).expect("Could not create Value");
        HttpResponse::Ok().json(json)
    } else {
        let json = serde_json::to_value(duplicates).expect("Could not create Value");
        HttpResponse::Found().json(json)
    }
}

#[delete("/{id}")]
async fn remove(path: web::Path<String>) -> impl Responder {
    let _id = path.into_inner();

    todo!();

    HttpResponse::Ok().finish()
}
