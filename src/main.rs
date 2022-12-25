use actix_web::{
    App, HttpServer,
    web::{self, ServiceConfig},
    middleware::Logger
};
use reqwest::{header::{HeaderMap, HeaderValue}, Client};
use log::info;

use kaspi_service::{
    spawn_save,
    routes::{
        products::{show_all, show, add, remove},
        code::{check_all, check},
    },
    STORE,
};


pub fn init(config: &mut ServiceConfig) {
    config
        .service(
            web::scope("/products")
                .service(show_all)
                .service(show)
                .service(add)
                .service(remove)
        )
        .service(
            web::scope("/code")
                .service(check_all)
                .service(check)
        );
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let api_key = dotenv::var("KASPI_API").expect("Kaspi API key is not provided");
    println!("Shirin");

    STORE.fill().await;
    info!("{} products", STORE.products().await.len());
    info!("{} entries waiting to be uploaded", STORE.uploaded_len().await);

    let mut headers = HeaderMap::new();
    headers.insert(
        "X-Auth-Token",
        HeaderValue::from_str(api_key.as_str()).expect("Could not create HeaderValue")
    );
    headers.insert(
        "Content-Type",
        HeaderValue::from_str("text/plain").expect("Could not create HeaderValue")
    );
    headers.insert(
        "Accept",
        HeaderValue::from_str("application/json").expect("Could not create HeaderValue")
    );

    let client = Client::builder().default_headers(headers).build()?;

    HttpServer::new(move ||
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(client.clone()))
            .configure(init))
            .bind("localhost:8000")?
            .run()
            .await?;

    spawn_save().await;

    Ok(())
}
