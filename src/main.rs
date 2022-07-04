use actix_web::{get, post, web::{self, ServiceConfig}, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use std::{
    fs,
    io::{self, Read, Write}
};
use kaspi_rs::product::Product;

const FILE_NAME: &'static str = "products.json";

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rust")
}

fn read_file(file_name: &str) -> io::Result<fs::File> {
    fs::File::open(file_name).or_else(|_| {
        let mut f = fs::File::create(file_name).expect("Could not create a json file");
        f.write_all(b"[]").expect("Could not populate json file");
        println!("Created a json file");

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

#[get("/products")]
async fn products() -> impl Responder {
    let json = open_json().expect("Could not open file");

    HttpResponse::Ok().json(json)
}

#[post("/add_product")]
async fn add_product(product: web::Json<Product>) -> impl Responder {
    let mut json = open_json().expect("Could not open file");

    let product_json = serde_json::to_value(product).expect("Could not convert to json");
    json.push(product_json);

    let j = serde_json::to_value(json).expect("Could not convert to Json<Value>");
    save_json(FILE_NAME, j).expect("Could not write json");

    HttpResponse::Ok()
}

pub fn init(config: &mut ServiceConfig) {
    config
        .service(
            web::scope("")
                .service(index)
                .service(products)
                .service(add_product)
        );
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move ||
        App::new()
            .wrap(Logger::default())
            .configure(init))
            .bind("localhost:8000")?
            .run()
            .await?;

    Ok(())
}
