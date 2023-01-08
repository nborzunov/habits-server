use actix_web::{web, Scope};

pub mod auth;
pub mod habits;
pub mod targets;
pub mod users;

pub fn routes() -> Scope {
    web::scope("")
        .service(habits::routes())
        .service(targets::routes())
        .service(auth::routes())
        .service(users::routes())
}
