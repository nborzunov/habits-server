use crate::achievements::models::{AchievementKey, AchievementType};
use crate::common::middlewares::auth::AuthenticationService;
use crate::targets::models::{Target, TargetData};
use crate::{achievements, habits, targets};
use actix_web::{post, web, HttpResponse, Scope};
use mongodb::Client;
use tokio::sync::mpsc;

pub fn routes() -> Scope {
    web::scope("/targets").service(create)
}

#[post("/")]
pub async fn create(
    achievements_data: web::Data<mpsc::UnboundedSender<Vec<AchievementKey>>>,
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
                tokio::spawn(achievements::repository::check_all(
                    client.clone(),
                    AchievementType::Habits,
                    form.habit_id,
                    achievements_data.get_ref().clone(),
                ));

                return HttpResponse::Ok().json(habit.clone());
            }
            Err(err) => HttpResponse::InternalServerError().body(err),
        },
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}
