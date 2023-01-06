use actix_web::{web, HttpResponse, get, post, delete};
use crate::models::habits::{HabitData, HabitDetails, Habit};
use mongodb::Client;
use crate::repository;
use std::iter::Iterator;
use crate::models::targets::TargetDetails;

#[get("/habits")]
pub async fn get_all(client: web::Data<Client>) -> HttpResponse {
    let habits: Vec<Habit> = repository::habits::get_all(client.clone()).await;

    let result = futures::future::join_all(
        habits.iter().map(|h| async {
            let targets = repository::targets::get_all(client.clone(), &h.id.clone().unwrap()).await;
            HabitDetails::parse(h, targets.iter().map(|t| TargetDetails::parse(&t)).collect())
        })
    ).await;

    HttpResponse::Ok().json(result)
}

#[post("/habits")]
pub async fn create(client: web::Data<Client>, form: web::Json<HabitData>) -> HttpResponse {
    let res = repository::habits::create(client, Habit::new(&form.into_inner())).await;

    match res {
        Ok(_) => HttpResponse::Ok().body("habit added"),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

#[delete("/habits/{habit_id}")]
pub async fn delete(client: web::Data<Client>, path: web::Path<String>) -> HttpResponse {
    let res = repository::habits::delete(client, path.into_inner()).await;

    match res {
        Ok(_) => HttpResponse::Ok().body("habit deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

// TODO: edit habit data