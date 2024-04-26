use crate::common::middlewares::auth::AuthenticationService;
use crate::features::habit::models::{Habit, HabitsAchievement};
use crate::features::habit_target::models::{Target, TargetData};
use crate::repository::database::Database;
use actix_web::{post, web, HttpResponse, Scope};
use tokio::sync::mpsc;

pub fn routes() -> Scope {
    web::scope("/targets").service(create).service(clean)
}

#[post("/")]
async fn create(
    achievements_data: web::Data<mpsc::UnboundedSender<Vec<String>>>,
    user: AuthenticationService,
    db: web::Data<Database>,
    form: web::Json<TargetData>,
) -> HttpResponse {
    match Target::insert(db.clone(), user.0.id, form.clone()).await {
        Ok(_) => match Habit::get_details(db.clone(), form.habit_id).await {
            Ok(habit) => {
                tokio::spawn(HabitsAchievement::check_all(
                    db.clone(),
                    achievements_data.get_ref().clone(),
                    user.0.id,
                    form.habit_id,
                ));

                return HttpResponse::Ok().json(habit.clone());
            }
            Err(err) => HttpResponse::InternalServerError().body(err),
        },
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

#[post("/clean")]
async fn clean(user: AuthenticationService, db: web::Data<Database>) -> HttpResponse {
    match Target::clean_data(db.clone(), user.0.id).await {
        Ok(_) => HttpResponse::Ok().body("targets cleaned"),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}
