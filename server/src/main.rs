#[macro_use]
extern crate diesel;

use std::env::set_var;

use actix_web::{App, HttpServer, web};
use diesel::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;

mod schema;
mod app;
mod db_con;
mod models;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let conn_url = std::env::var("DATABASE_URL").expect("Failed to get value of DATABASE_URL");

    // let private_key = match fs::read("cookie-key") {
    //     Ok(bytes) => bytes,
    //     Err(e) => {
    //         if e.kind() == io::ErrorKind::NotFound {
    //             let mut f =
    //                 fs::File::create("cookie-key").expect("Unable to create cookie key file");
    //             let key: [u8; 32] = rand::random();
    //
    //             f.write_all(&key).expect("Unable to write to file");
    //             key.to_vec()
    //         } else {
    //             panic!(e)
    //         }
    //     }
    // };

    let manager = ConnectionManager::<MysqlConnection>::new(&conn_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            // .wrap(IdentityService::new(
            //     CookieIdentityPolicy::new(&private_key)
            //         .name("actix-web-example")
            //         .secure(false)
            //         .max_age(31_556_952),
            // ))
            .app_data(pool.clone())
            .service(app::habits::get_habits)
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
