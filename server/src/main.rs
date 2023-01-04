//#[actix_web::main]
//async fn main() -> std::io::Result<()> {
//    std::env::set_var("RUST_LOG", "debug");
//    env_logger::init();
//    HttpServer::new(|| App::new().configure(app::config))
//        .bind(("127.0.0.1", 8080))?
//        .run()
//        .await
//}

use actix_web::{App, HttpServer};
use dotenv::dotenv;

use std::env::set_var;

mod app;
mod db_con;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    set_var("RUST_LOG", "actix_web=debug");

    let pool = db_con::get_pool();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(app::habits::get_habits)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
