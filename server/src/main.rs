use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;

use std::env::{set_var, var};

mod routes;
mod models;
mod repository;

use mongodb::{
    bson::{extjson::de::Error},
    results::{ InsertOneResult},
    Client, Collection,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env logger
    env_logger::init();
    dotenv().ok();
    let uri = match var("DATABASE_URL") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };
    let client = Client::with_uri_str(uri).await.unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.clone()))
            .service(routes::habits::get_habits)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}