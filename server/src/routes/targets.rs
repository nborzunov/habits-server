use actix_web::{HttpResponse, post, web};
use mongodb::Client;

use crate::models::habits::HabitDetails;
use crate::models::targets::{Target, TargetData, TargetDetails};
use crate::repository;

#[post("/targets")]
pub async fn create(client: web::Data<Client>, form: web::Json<TargetData>) -> HttpResponse {
    let res = repository::targets::create(client.clone(), Target::new(&form.clone())).await;

    match res {
        Ok(_) => {
            let habit = repository::habits::get_by_id(client.clone(), form.habit_id.to_string()).await;
            let targets = repository::targets::get_all(client.clone(), &habit.id.clone().unwrap()).await;

            HttpResponse::Ok().json(HabitDetails::parse(&habit, targets.iter().map(|t| TargetDetails::parse(&t)).collect()))
        }
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

// TODO: add target
// TODO: change target type