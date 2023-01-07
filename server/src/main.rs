use std::env::{set_var, var};

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger};
use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use mongodb::Client;

mod models;
mod repository;
mod routes;

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
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .app_data(Data::new(client.clone()))
            .service(routes::habits::get_all)
            .service(routes::habits::create)
            .service(routes::habits::edit)
            .service(routes::habits::delete)
            .service(routes::habits::archive)
            .service(routes::targets::create)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
