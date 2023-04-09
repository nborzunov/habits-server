use std::env::{set_var, var};
use std::sync::{Arc, Mutex};

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger};
use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use mongodb::Client;

use crate::achievements::models::AchievementKey;
use lazy_static::lazy_static;
use tokio::sync::mpsc;

mod achievements;
mod auth;
mod common;
mod habits;
mod routes;
mod targets;
mod users;

lazy_static! {
    pub static ref DB_NAME: String = match var("DB_NAME") {
        Ok(v) => v,
        Err(_) => panic!("Error loading DB_NAME variable"),
    };
}

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

    let (achievements_sender, achievements_receiver) =
        mpsc::unbounded_channel::<Vec<AchievementKey>>();
    let achievements_receiver = Arc::new(Mutex::new(achievements_receiver));

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
            .app_data(Data::new(achievements_sender.clone()))
            .app_data(Data::new(achievements_receiver.clone()))
            .service(routes::routes())
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
