use crate::common::middlewares::auth::AuthenticationService;
use crate::features::habit_target::models::{Target, TargetData};
use crate::repository::database::Database;
use actix_web::{delete, post, web, HttpResponse, Scope};
use tokio::sync::mpsc;
use uuid::Uuid;

pub fn routes() -> Scope {
    web::scope("/targets")
        .service(create_target)
        .service(delete_target)
        .service(clean_targets)
}

#[post("/")]
async fn create_target(
    _achievements_data: web::Data<mpsc::UnboundedSender<Vec<String>>>,
    user: AuthenticationService,
    db: web::Data<Database>,
    form: web::Json<TargetData>,
) -> HttpResponse {
    match Target::insert(db.clone(), user.0.id, form.clone()).await {
        Ok(_) => {
            // tokio::spawn(HabitsAchievement::check_all(
            //     db.clone(),
            //     achievements_data.get_ref().clone(),
            //     user.0.id,
            //     form.habit_id,
            // ));

            return HttpResponse::Ok().body("target created");
        }
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

#[delete("/{target_id}")]
async fn delete_target(
    _: AuthenticationService,
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    match Target::delete(db.clone(), path.clone()).await {
        Ok(_) => HttpResponse::Ok().body("target deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

#[post("/clean")]
async fn clean_targets(user: AuthenticationService, db: web::Data<Database>) -> HttpResponse {
    match Target::clean_data(db.clone(), user.0.id).await {
        Ok(_) => HttpResponse::Ok().body("targets cleaned"),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}
