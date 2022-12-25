use actix_web::{get, web, Responder, HttpResponse, HttpResponseBuilder, http::StatusCode};
use reqwest::Client;
use futures::future;
use serde_json::json;
use uuid::Uuid;
use crate::{
    STORE,
    check_code,
};

#[get("/{id}")]
async fn check(path: web::Path<String>, client: web::Data<Client>) -> impl Responder {
    let id = path.as_str();

    let result = check_code(
        &Uuid::parse_str(id).unwrap(),
        client.into_inner()
    ).await;

    if let Ok(response_json) = result {
        HttpResponse::Ok().json(response_json)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[get("/")]
async fn check_all(client: web::Data<Client>) -> impl Responder {
    let result: Vec<serde_json::Value> = future::join_all(
        STORE.uploaded_ids().await.iter().map(|id| {
            check_code(
                id,
                client.clone().into_inner()
            )
        }
    )).await
        .into_iter()
        .map(|r| json!(r.unwrap()))
        .collect();

    HttpResponse::Ok().json(serde_json::to_value(result).expect("Could not create json"))
}
