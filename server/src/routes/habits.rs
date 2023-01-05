use actix_web::{web, HttpResponse, get, post, delete};
use crate::models::habits::{HabitData, HabitDetails, Habit};
use mongodb::Client;
use crate::repository;

#[get("/habits")]
pub async fn get_all(client: web::Data<Client>) -> HttpResponse {
    let habits: Vec<HabitDetails> = repository::habits::get_all(client).await
        .iter().map(|h| h.get_details()).collect();
    HttpResponse::Ok().json(habits)
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

// #[post("/")]
// pub async fn add_habit(
//         request: web::Json<HabitModel>,
//         ) -> SerdeResult<HttpResponse> {
//     let habit_model = request.into_inner();
//
//     let new_habit = Habit::new(&habit_model);
//
//     println!("{:?}", new_habit);
//
//     Ok(HttpResponse::Ok().json(new_habit))
// }


// TODO: add target
// TODO: change target type
// TODO: edit habit data