use std::env::{set_var, var};

use actix_cors::Cors;
use actix_web::{ http::header, middleware::Logger};
use actix_web::{App, HttpServer, web::Data};
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
    let port: i32 = var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");
    // let url = var("URL").unwrap_or_else(|_| "127.0.0.1".to_string());

    let client = Client::with_uri_str(uri).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin_fn(|_, _| true)
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
        //("127.0.0.1", 8080)
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await
}
