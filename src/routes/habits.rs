use std::iter::Iterator;
use std::str::FromStr;

use actix_web::{delete, get, post, put, web, HttpResponse, Scope};
use mongodb::bson::oid::ObjectId;
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
        .service(clean_habit)
        .service(clean_habits)
        .service(delete_habits)
}

#[get("/")]
pub async fn get_all(user: AuthenticationService, client: web::Data<Client>) -> HttpResponse {
    let habits = match repository::habits::get_all(client.clone(), user.0.id.unwrap()).await {
        Ok(habits) => habits,
        Err(err) => return HttpResponse::InternalServerError().body(err),
    };

    let result = futures::future::join_all(habits.iter().map(|h| async {
        let targets =
            match repository::targets::get_all(client.clone(), &h.id.clone().unwrap()).await {
                Ok(targets) => targets.iter().map(|t| TargetDetails::parse(t)).collect(),
                Err(err) => return Err(err),
            };
        Ok(HabitDetails::parse(h, targets))
    }))
    .await
    .into_iter()
    .collect::<Result<Vec<HabitDetails>, String>>();

    match result {
        Ok(habits) => HttpResponse::Ok().json(habits),
        Err(err) => return HttpResponse::InternalServerError().body(err),
    }
}

#[post("/")]
pub async fn create(
    user: AuthenticationService,
    client: web::Data<Client>,
    form: web::Json<HabitData>,
) -> HttpResponse {
    match repository::habits::create(
        client.clone(),
        Habit::new(&form.into_inner(), user.0.id.unwrap()),
    )
    .await
    {
        Ok(habit_id) => match repository::habits::get_details(client.clone(), habit_id).await {
            Ok(habit) => HttpResponse::Ok().json(habit),
            Err(err) => HttpResponse::InternalServerError().body(err),
        },

        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

#[put("/{habit_id}")]
pub async fn edit(
    user: AuthenticationService,
    client: web::Data<Client>,
    path: web::Path<String>,
    form: web::Json<HabitData>,
) -> HttpResponse {
    let habit_id = ObjectId::from_str(&path.clone()).unwrap();
    match repository::habits::edit(
        client.clone(),
        user.0.id.unwrap(),
        habit_id.clone(),
        form.into_inner(),
    )
    .await
    {
        Ok(_) => match repository::habits::get_details(client.clone(), habit_id).await {
            Ok(habit) => HttpResponse::Ok().json(habit),
            Err(err) => HttpResponse::InternalServerError().body(err),
        },
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

#[delete("/{habit_id}")]
pub async fn delete(
    user: AuthenticationService,
    client: web::Data<Client>,
    path: web::Path<String>,
) -> HttpResponse {
    let habit_id = ObjectId::from_str(&path.clone()).unwrap();
    let res = repository::habits::delete(client, user.0.id.unwrap(), habit_id).await;

    match res {
        Ok(_) => HttpResponse::Ok().body("habit deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

#[put("/{habit_id}/archive")]
pub async fn archive(
    user: AuthenticationService,
    client: web::Data<Client>,
    path: web::Path<String>,
) -> HttpResponse {
    let habit_id = ObjectId::from_str(&path.clone()).unwrap();
    let res = repository::habits::archive(client, user.0.id.unwrap(), habit_id).await;
    match res {
        Ok(_) => HttpResponse::Ok().body("habit archived"),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

#[put("/{habit_id}/clean")]
pub async fn clean_habit(
    _: AuthenticationService,
    client: web::Data<Client>,
    path: web::Path<String>,
) -> HttpResponse {
    let habit_id = ObjectId::from_str(&path.clone()).unwrap();
    let res = repository::targets::clean_data(client, &habit_id).await;
    match res {
        Ok(_) => HttpResponse::Ok().body("data cleaned"),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

#[post("/clean")]
pub async fn clean_habits(user: AuthenticationService, client: web::Data<Client>) -> HttpResponse {
    match repository::habits::clean_data(client, user.0.id.unwrap()).await {
        Ok(_) => HttpResponse::Ok().body("habits cleaned"),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

#[delete("/")]
pub async fn delete_habits(user: AuthenticationService, client: web::Data<Client>) -> HttpResponse {
    match repository::habits::delete_all_habits(client, user.0.id.unwrap()).await {
        Ok(_) => HttpResponse::Ok().body("habits deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}
