use actix_web::{web, HttpResponse, Scope};

pub mod auth;
pub mod habits;
pub mod targets;
pub mod users;

async fn init() -> HttpResponse {
    HttpResponse::Ok().body("Server works!")
}

pub fn routes() -> Scope {
    web::scope("")
        .service(habits::routes())
        .service(habits::routes())
        .service(targets::routes())
        .service(auth::routes())
        .service(users::routes())
        .route("/", web::get().to(init))
}
