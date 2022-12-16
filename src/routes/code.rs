use actix_web::{get, web, Responder, HttpResponseBuilder, http::StatusCode};
use reqwest::Client;
use futures::future;
use serde_json::json;
use crate::check_code;
use crate::json_processing::*;

#[get("/{id}")]
async fn check(path: web::Path<String>, client: web::Data<Client>) -> impl Responder {
    let id = path.into_inner();

    let json = open_json().expect("Could not open file");
    let product = json.into_iter().find(|p| p["id"].eq(&id)).expect("Could not find product with such id");

    let response_json = check_code(
        &id,
        product["code"].as_str().unwrap(),
        client.into_inner()
    ).await;

    HttpResponseBuilder::new(StatusCode::OK).json(response_json)
}

#[get("/")]
async fn check_all(client: web::Data<Client>) -> impl Responder {
    let json = open_json().expect("Could not open file");

    let result: Vec<serde_json::Value> = future::join_all(
        json.iter().map(|p| {
            check_code(
                p["id"].as_str().unwrap(),
                p["code"].as_str().unwrap(),
                client.clone().into_inner()
            )
        }
    )).await
        .iter()
        .map(|r| json!(r))
        .collect();

    HttpResponseBuilder::new(StatusCode::OK).json(serde_json::to_value(result).expect("Could not create json"))
}
