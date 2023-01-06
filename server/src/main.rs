use std::env::{set_var, var};

use actix_web::{App, HttpServer, web::Data};
use actix_web::{http::header, middleware::Logger};
use dotenv::dotenv;
use mongodb::Client;

use actix_cors::Cors;

mod routes;
mod models;
mod repository;

static DB_NAME: &str = "dev";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv().ok();
    let uri = match var("DATABASE_URL") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };
    let client = Client::with_uri_str(uri).await.unwrap();


    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default()
                .allowed_origin("http://localhost:3000")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .supports_credentials()
                .max_age(3600))
            .wrap(Logger::default())
            .app_data(Data::new(client.clone()))
            .service(routes::habits::get_all)
            .service(routes::habits::create)
            .service(routes::habits::delete)
            .service(routes::targets::create)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}