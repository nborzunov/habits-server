use crate::features;
use actix_web::{web, HttpResponse, Scope};

async fn init() -> HttpResponse {
    HttpResponse::Ok().body("Server works!")
}

pub fn routes() -> Scope {
    web::scope("")
        .route("/", web::get().to(init))
        .service(features::auth::handlers::routes())
        .service(features::user::handlers::routes())
        .service(features::habit::handlers::routes())
        .service(features::habit_target::handlers::routes())
        .service(features::achievement::handlers::routes())
        .service(features::account::handlers::routes())
        .service(features::category::handlers::routes())
        .service(features::transaction::handlers::routes())
}
