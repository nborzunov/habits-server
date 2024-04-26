use std::env::{set_var, var};
use std::sync::{Arc, Mutex};

use actix_cors::Cors;
use actix_web::web;
use actix_web::{http::header, middleware::Logger};
use actix_web::{web::Data, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenv::dotenv;

use tokio::sync::mpsc;

#[macro_use]
extern crate diesel;

mod common;
mod features;
mod repository;
mod routes;
mod schema;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pub pool: DBPool,
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv().ok();

    let port: i32 = var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let (achievements_sender, achievements_receiver) = mpsc::unbounded_channel::<Vec<String>>();
    let achievements_receiver = Arc::new(Mutex::new(achievements_receiver));

    let db = repository::database::Database::new();
    let app_data = web::Data::new(db);

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
            .app_data(app_data.clone())
            .app_data(Data::new(achievements_sender.clone()))
            .app_data(Data::new(achievements_receiver.clone()))
            .service(routes::routes())
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
