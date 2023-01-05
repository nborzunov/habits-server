use actix_web::{web, HttpResponse, get, post};
use serde_json;
use serde_json::Result as SerdeResult;
use crate::models::habits::{HabitModel, Habit};
use mongodb::{Client, Collection, IndexModel};
use mongodb::bson::doc;
use mongodb::options::IndexOptions;
use futures::stream::TryStreamExt;
use crate::repository;


#[get("/")]
pub async fn get_habits(client: web::Data<Client>) -> SerdeResult<HttpResponse> {
    let habits= repository::habits::get_all(client).await;
    Ok(HttpResponse::Ok().json(habits))
}

// #[get("/{id}")]
// pub async fn get_habit(client: web::Data<Client>) -> SerdeResult<HttpResponse> {
//     match Habit::get_habit(client).await {
//         Ok(v) => Ok(HttpResponse::Ok().json(v)),
//         Err(e) => {
//             println!("{:?}", e);
//             return Ok(HttpResponse::InternalServerError().body(format!("Server error")))
//         },
//     }
// }

#[post("/")]
pub async fn add_habit(
        request: web::Json<HabitModel>,
        ) -> SerdeResult<HttpResponse> {
    let habit_model = request.into_inner();

    let new_habit = Habit::new(&habit_model);

    println!("{:?}", new_habit);

    Ok(HttpResponse::Ok().json(new_habit))
}


// TODO: add target
// TODO: change target type
// TODO: edit habit data