use actix_web::{get, post, web, HttpResponse, Responder, HttpResponseBuilder, http::StatusCode, delete};
use reqwest::Client;
use serde_json::json;
use crate::{
    send_to_kaspi,
    json_processing::*,
    entities::product::{Product},
};

#[get("/")]
async fn show_all() -> impl Responder {
    let json = open_json().expect("Could not open file");

    let mut result: Vec<serde_json::Value> = Vec::new();
    for v in json.iter() {
        let entry = json!({
            "id": v["id"],
            "sku": v["product"]["sku"],
            "code": v["code"],
            "status": v["status"]
        });
        result.push(entry);
    }

    HttpResponse::Ok().json(result)
}

#[get("/{id}")]
async fn show(path: web::Path<String>) -> impl Responder {
    let json = open_json().expect("Could not open file");
    let id = path.into_inner();

    if let Some(pos) = json.iter().position(|p| p["id"].eq(&id)) {
        let p = json.get(pos).unwrap();

        HttpResponse::Ok().json(p)
    } else {
        HttpResponseBuilder::new(StatusCode::from_u16(500).unwrap()).finish()
    }
}

#[post("/")]
async fn add(products: web::Json<Vec<Product>>, client: web::Data<Client>) -> impl Responder {
    let mut json = open_json().expect("Could not open file");

    for product in products.into_inner().iter() {

        let kaspi_product = product.clone();
        let product_json = serde_json::to_value(kaspi_product.clone()).expect("Could not convert to json");

        let duplicate = json
            .iter()
            .filter(|p| p["sku"].eq(&product_json["sku"]))
            .collect::<Vec<&serde_json::Value>>()
            .is_empty() ^ true;

        if duplicate {
            return HttpResponseBuilder::new(StatusCode::from_u16(500).unwrap())
                .body(format!("Duplicate found: {}", product_json["sku"]))
        } else {
            let record = send_to_kaspi(
                product_json, client.clone().into_inner()
            ).await.expect("Error while sending product to kaspi");

            json.push(serde_json::to_value(record).unwrap());
        }
    }
    let j = serde_json::to_value(json).expect("Could not convert to Json<Value>");
    save_json(FILE_NAME, j).expect("Could not write json");

    HttpResponseBuilder::new(StatusCode::from_u16(200).unwrap()).finish()
}

#[delete("/{id}")]
async fn remove(path: web::Path<String>) -> impl Responder {
    let mut json = open_json().expect("Could not open file");
    let size_before = json.len();
    let id = path.into_inner();

    json.retain(|p| {
        p["id"].ne(&id)
    });

    if size_before == json.len() {
        HttpResponseBuilder::new(StatusCode::from_u16(500).unwrap()).body("ID not found")
    } else {
        save_json(
            FILE_NAME,
            serde_json::to_value(json).expect("Could not convert to Json<Value>")
        ).expect("Could not write json");

        HttpResponse::Ok().body("")
    }
}
