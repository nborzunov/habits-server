use crate::{achievements, auth, finance, habits, targets, users};
use actix_web::{web, HttpResponse, Scope};

async fn init() -> HttpResponse {
    HttpResponse::Ok().body("Server works!")
}

pub fn routes() -> Scope {
    web::scope("")
        .service(auth::routes::routes())
        .service(users::routes::routes())
        .service(habits::routes::routes())
        .service(targets::routes::routes())
        .service(achievements::routes::routes())
        .service(finance::routes::routes())
        .route("/", web::get().to(init))
}
