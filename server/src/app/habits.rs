use actix_web::{get, HttpResponse, post, web};
use serde_json;
use serde_json::Result as SerdeResult;

use crate::DbPool;

// use crate::models::habits::{HabitModel, Habit};

#[get("/")]
pub async fn get_habits(pool: web::Data<DbPool>) -> SerdeResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(""))
}

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
//     // TODO: handle if there is habit with the same name
// //    state.habits.inner().lock().unwrap().push(new_habit.clone());
//     Ok(HttpResponse::Ok().json(new_habit))
// }


// TODO: add target
// TODO: change target type
// TODO: edit habit data