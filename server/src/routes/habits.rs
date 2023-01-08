use std::iter::Iterator;
use std::str::FromStr;

use actix_web::{delete, get, post, put, web, HttpResponse, Scope};
use mongodb::Client;

use crate::middlewares::auth::AuthenticationService;
use crate::models::habits::{Habit, HabitData, HabitDetails};
use crate::models::targets::TargetDetails;
use crate::repository;

pub fn routes() -> Scope {
    web::scope("/habits")
        .service(get_all)
        .service(create)
        .service(edit)
        .service(delete)
        .service(archive)
}

#[get("/")]
pub async fn get_all(_: AuthenticationService, client: web::Data<Client>) -> HttpResponse {
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

#[post("/")]
pub async fn create(
    _: AuthenticationService,
    client: web::Data<Client>,
    form: web::Json<HabitData>,
) -> HttpResponse {
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

#[put("/{habit_id}")]
pub async fn edit(
    _: AuthenticationService,
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

#[delete("/{habit_id}")]
pub async fn delete(
    _: AuthenticationService,
    client: web::Data<Client>,
    path: web::Path<String>,
) -> HttpResponse {
    let res = repository::habits::delete(client, path.into_inner()).await;

    match res {
        Ok(_) => HttpResponse::Ok().body("habit deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

#[put("/{habit_id}/archive")]
pub async fn archive(
    _: AuthenticationService,
    client: web::Data<Client>,
    path: web::Path<String>,
) -> HttpResponse {
    let res = repository::habits::archive(client, path.into_inner()).await;
    match res {
        Ok(_) => HttpResponse::Ok().body("habit archived"),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}
