use actix_web::{web, Scope};

pub mod habits;
pub mod targets;

pub fn routes() -> Scope {
    web::scope("")
        .service(habits::routes())
        .service(targets::routes())
}
