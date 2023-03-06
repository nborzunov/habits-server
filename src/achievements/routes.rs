use actix_web::{get, web, HttpResponse, Scope};

use mongodb::Client;

use crate::common::middlewares::auth::AuthenticationService;
use crate::habits::models::{HabitDetails, HabitsAchievement};
use crate::{achievements, habits};
use serde::{Deserialize, Serialize};

pub fn routes() -> Scope {
    web::scope("/achievements").service(get)
}

#[derive(Serialize, Deserialize)]
struct Response {
    habit: HabitDetails,
    achievements: Vec<HabitsAchievement>,
}
#[get("/")]
pub async fn get(user: AuthenticationService, client: web::Data<Client>) -> HttpResponse {
    let habits = habits::repository::get_all(client.clone(), user.0.id.unwrap())
        .await
        .unwrap();

    let result = achievements::repository::get_all(client.clone(), user.0.id.unwrap(), habits)
        .await
        .unwrap();
    HttpResponse::Ok().json(result)
}
