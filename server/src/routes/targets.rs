use actix_web::{post, web, HttpResponse};
use mongodb::Client;

use crate::models::targets::{Target, TargetData};
use crate::repository;

#[post("/targets")]
pub async fn create(client: web::Data<Client>, form: web::Json<TargetData>) -> HttpResponse {
    let res = repository::targets::create(client.clone(), Target::new(&form.clone())).await;

    match res {
        Ok(_) => HttpResponse::Ok()
            .json(repository::habits::get_details(client.clone(), form.habit_id).await),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

// TODO: add target
// TODO: change target type
