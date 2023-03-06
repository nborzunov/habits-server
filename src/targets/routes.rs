use actix_web::{post, web, HttpResponse, Scope};
use mongodb::Client;

use crate::achievements::models::{AchievementKey, AchievementType};
use crate::common::middlewares::auth::AuthenticationService;
use crate::habits::models::HabitDetails;
use crate::targets::models::{Target, TargetData};
use crate::{achievements, habits, targets};
use serde::{Deserialize, Serialize};

pub fn routes() -> Scope {
    web::scope("/targets").service(create)
}

#[derive(Serialize, Deserialize)]
struct Response {
    habit: HabitDetails,
    achievements: Vec<AchievementKey>,
}
#[post("/")]
pub async fn create(
    user: AuthenticationService,
    client: web::Data<Client>,
    form: web::Json<TargetData>,
) -> HttpResponse {
    match targets::repository::create(
        client.clone(),
        user.0.id.unwrap(),
        Target::new(&form.clone()),
    )
    .await
    {
        Ok(_) => match habits::repository::get_details(client.clone(), form.habit_id).await {
            Ok(habit) => {
                let new_achievements = achievements::repository::check_all(
                    client.clone(),
                    AchievementType::Habits,
                    form.habit_id,
                )
                .await;

                return HttpResponse::Ok().json(Response {
                    habit: habit.clone(),
                    achievements: new_achievements,
                });
            }
            Err(err) => HttpResponse::InternalServerError().body(err),
        },
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}
