use std::iter::Iterator;
use std::str::FromStr;

use actix_web::{delete, get, post, put, web, HttpResponse};
use mongodb::Client;

use crate::models::habits::{Habit, HabitData, HabitDetails};
use crate::models::targets::TargetDetails;
use crate::repository;

#[get("/habits")]
pub async fn get_all(client: web::Data<Client>) -> HttpResponse {
    let habits: Vec<Habit> = repository::habits::get_all(client.clone()).await;

    let result = futures::future::join_all(habits.iter().map(|h| async {
        let targets = repository::targets::get_all(client.clone(), &h.id.clone().unwrap()).await;
        HabitDetails::parse(
            h,
            targets.iter().map(|t| TargetDetails::parse(&t)).collect(),
        )
    }))
    .await;

    HttpResponse::Ok().json(result)
}

#[post("/habits")]
pub async fn create(client: web::Data<Client>, form: web::Json<HabitData>) -> HttpResponse {
    let res = repository::habits::create(client.clone(), Habit::new(&form.into_inner())).await;

    match res {
        Ok(habit_id) => {
            let habit =
                repository::habits::get_details(client.clone(), habit_id.as_object_id().unwrap())
                    .await;
            HttpResponse::Ok().json(habit)
        }
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

#[put("/habits/{habit_id}")]
pub async fn edit(
    client: web::Data<Client>,
    path: web::Path<String>,
    form: web::Json<HabitData>,
) -> HttpResponse {
    repository::habits::edit(client.clone(), path.clone(), form.into_inner()).await;
    let habit = repository::habits::get_details(
        client.clone(),
        mongodb::bson::oid::ObjectId::from_str(&path.clone()).unwrap(),
    )
    .await;

    HttpResponse::Ok().json(habit)
}

#[delete("/habits/{habit_id}")]
pub async fn delete(client: web::Data<Client>, path: web::Path<String>) -> HttpResponse {
    let res = repository::habits::delete(client, path.into_inner()).await;

    match res {
        Ok(_) => HttpResponse::Ok().body("habit deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

#[put("/habits/{habit_id}/archive")]
pub async fn archive(client: web::Data<Client>, path: web::Path<String>) -> HttpResponse {
    let res = repository::habits::archive(client, path.into_inner()).await;
    match res {
        Ok(_) => HttpResponse::Ok().body("habit archived"),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}
