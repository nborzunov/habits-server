use std::iter::Iterator;

use actix_web::{delete, get, HttpResponse, post, web};
use mongodb::Client;

use crate::models::habits::{Habit, HabitData, HabitDetails};
use crate::models::targets::TargetDetails;
use crate::repository;

#[get("/habits")]
pub async fn get_all(client: web::Data<Client>) -> HttpResponse {
    let habits: Vec<Habit> = repository::habits::get_all(client.clone()).await;

    let result = futures::future::join_all(
        habits.iter().map(|h| async {
            let targets = repository::targets::get_all(client.clone(), &h.id.clone().unwrap())
                .await;
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