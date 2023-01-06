use actix_web::{web, HttpResponse, get, post, delete};
use crate::models::habits::{HabitData, HabitDetails, Habit};
use mongodb::Client;
use crate::models::targets::{Target, TargetData};
use crate::repository;

#[post("/targets")]
pub async fn create(client: web::Data<Client>, form: web::Json<TargetData>) -> HttpResponse {
    let res = repository::targets::create(client, Target::new(&form.into_inner())).await;

    match res {
        Ok(_) => HttpResponse::Ok().body("target added"),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

// TODO: add target
// TODO: change target type