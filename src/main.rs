mod lib;

use actix_web::{get, post, put, web::{self, ServiceConfig}, App, HttpResponse, HttpServer, Responder, middleware::Logger, HttpResponseBuilder, http::StatusCode, delete};
use reqwest::{header::{HeaderMap, HeaderValue}, Client};
use serde_json::json;
use std::{
    fs,
    io::{self, Read, Write}
};
use futures::future;
use kaspi_rs::product::Product;
use lib::{check_code, send_to_kaspi};

const FILE_NAME: &'static str = "products.json";

#[get("")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rust")
}

fn read_file(file_name: &str) -> io::Result<fs::File> {
    fs::File::open(file_name).or_else(|_| {
        let mut f = fs::File::create(file_name).expect("Could not create a json file");
        f.write_all(b"[]").expect("Could not populate json file");

        Ok(f)
    })
}

fn open_json() -> serde_json::Result<Vec<serde_json::Value>> {
    let file = read_file(FILE_NAME).expect("Could not read a json file");

    let mut buf_reader = io::BufReader::new(&file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents).expect("Could not process a json file");

    let a: Vec<serde_json::Value> = serde_json::from_str(contents.as_str())?;

    Ok(a)
}

fn save_json(file_name: &str, json: serde_json::Value) -> io::Result<()> {
    let mut file = fs::File::create(file_name).expect("Could not create a file");

    file.write_all(json.to_string().as_bytes())?;
    Ok(())
}

#[get("")]
async fn show() -> impl Responder {
    let json = open_json().expect("Could not open file");

    HttpResponse::Ok().json(json)
}

#[post("")]
async fn add(products: web::Json<Vec<Product>>, client: web::Data<Client>) -> impl Responder {
    let mut json = open_json().expect("Could not open file");

    for mut product in products.into_inner().into_iter() {
        product.set_id();

        let mut product_json = serde_json::to_value(product).expect("Could not convert to json");
        let duplicate = json
            .iter()
            .filter(|p| p["sku"].eq(&product_json["sku"]))
            .collect::<Vec<&serde_json::Value>>()
            .is_empty() ^ true;

        if duplicate {
            return HttpResponseBuilder::new(StatusCode::from_u16(500).unwrap()).body(format!("Duplicate found: {}", product_json["sku"]))
        } else {
            product_json = send_to_kaspi(product_json, client.clone().into_inner()).await.expect("Error while sending product to kaspi");
            json.push(product_json);

        }
    }
    let j = serde_json::to_value(json).expect("Could not convert to Json<Value>");
    save_json(FILE_NAME, j).expect("Could not write json");

    HttpResponse::Ok().body("")
}

#[put("/{sku}")]
async fn update(product: web::Json<Product>, path: web::Path<String>) -> impl Responder {
    let mut json = open_json().expect("Could not open file");
    let sku = path.into_inner();

    if let Some(pos) = json.iter().position(|p| p["sku"].eq(&sku)) {
        let p = json.get_mut(pos).unwrap();
        *p = serde_json::to_value(product).expect("Could not convert to json");

        save_json(
            FILE_NAME,
            serde_json::to_value(json).expect("Could not convert to Json<Value>")
        ).expect("Could not write json");

        HttpResponse::Ok()
    } else {
        HttpResponseBuilder::new(StatusCode::from_u16(500).unwrap())
    }
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

#[get("/{id}")]
async fn check(path: web::Path<String>, client: web::Data<Client>) -> impl Responder {
    let id = path.into_inner();

    let json = open_json().expect("Could not open file");
    let product = json.into_iter().find(|p| p["id"].eq(&id)).expect("Could not find product with such id");

    let response_json = check_code(
        product["code"].as_str().unwrap(),
        client.into_inner()
    ).await;

    HttpResponseBuilder::new(StatusCode::OK).json(response_json)
}



#[get("")]
async fn check_all(client: web::Data<Client>) -> impl Responder {
    let json = open_json().expect("Could not open file");

    let result: Vec<serde_json::Value> = future::join_all(
        json.iter().map(|p| {
            check_code(
                p["code"].as_str().unwrap(),
                client.clone().into_inner()
            )
        }
    )).await
        .iter()
        .map(|r| r.get("result").or(Some(&json!({
            "code": r["code"],
            "status": "UPLOADED"
        }))).expect("Something went wrong").to_owned())
        .collect();

    HttpResponseBuilder::new(StatusCode::OK).json(serde_json::to_value(result).expect("Could not create json"))
}

pub fn init(config: &mut ServiceConfig) {
    config
        .service(index)
        .service(
            web::scope("/products")
                .service(show)
                .service(add)
                .service(remove)
                .service(update)
        )
        .service(
            web::scope("/code")
                .service(check)
                .service(check_all)
        );
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let api_key = dotenv::var("KASPI_API").expect("Kaspi API key is not provided");

    let mut headers = HeaderMap::new();
    headers.insert("X-Auth-Token", HeaderValue::from_str(api_key.as_str()).expect("Could not create HeaderValue"));
    headers.insert("Content-Type", HeaderValue::from_str("text/plain").expect("Could not create HeaderValue"));
    headers.insert("Accept", HeaderValue::from_str("application/jason").expect("Could not create HeaderValue"));

    let client = Client::builder().default_headers(headers).build()?;

    HttpServer::new(move ||
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(client.clone()))
            .configure(init))
            .bind("localhost:8000")?
            .run()
            .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::{json, from_value};
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct ABC {
        a: String
    }

    #[test]
    fn vec() {
        let json_a = json!([{
            "a": "b"
        }]);
        let typed_a = from_value::<Vec<ABC>>(json_a).expect("Could not parse json");

        assert_eq!(typed_a, vec![ABC { a: String::from("b") }]);
    }
}
