use std::env::{set_var, var};

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger};
use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use mongodb::Client;

mod middlewares;
mod models;
mod repository;
mod routes;
mod services;

static DB_NAME: &str = "dev";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv().ok();
    let uri = match var("DATABASE_URL") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading DATABASE_URL variable"),
    };

    let port = var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

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
            .service(routes::routes())
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
