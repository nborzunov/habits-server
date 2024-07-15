use actix_web::{delete, get, post, put, web, HttpResponse, Scope};
use uuid::Uuid;

use crate::common::middlewares::auth::AuthenticationService;
use crate::features::habit::models::{Habit, HabitData, NewHabit};
use crate::features::habit_target::models::Target;
use crate::repository::database::Database;

pub fn routes() -> Scope {
    web::scope("/habits")
        .service(get_all)
        .service(create)
        .service(edit)
        .service(delete)
        .service(archive)
        .service(clean_habit)
        .service(delete_habits)
        .service(get_todays_habits)
        .service(get_grid_habits)
}

#[get("/")]
async fn get_all(user: AuthenticationService, db: web::Data<Database>) -> HttpResponse {
    match Habit::get_all(db.clone(), user.0.id).await {
        Ok(habits) => HttpResponse::Ok().json(habits),
        Err(err) => return HttpResponse::InternalServerError().body(err),
    }
}

#[get("/today")]
async fn get_todays_habits(user: AuthenticationService, db: web::Data<Database>) -> HttpResponse {
    match Habit::get_todays_habits(db.clone(), user.0.id).await {
        Ok(habits) => HttpResponse::Ok().json(habits),
        Err(err) => return HttpResponse::InternalServerError().body(err),
    }
}

#[get("/grid")]
async fn get_grid_habits(user: AuthenticationService, db: web::Data<Database>) -> HttpResponse {
    match Habit::get_grid_habits(db.clone(), user.0.id).await {
        Ok(habits) => HttpResponse::Ok().json(habits),
        Err(err) => return HttpResponse::InternalServerError().body(err),
    }
}

#[post("/")]
async fn create(
    user: AuthenticationService,
    db: web::Data<Database>,
    form: web::Json<HabitData>,
) -> HttpResponse {
    match Habit::create(db.clone(), NewHabit::create(form.into_inner(), user.0.id)).await {
        Ok(habit_id) => match Habit::get_details(db.clone(), habit_id).await {
            Ok(habit) => HttpResponse::Ok().json(habit),
            Err(err) => HttpResponse::InternalServerError().body(err),
        },

        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

#[put("/{habit_id}")]
async fn edit(
    _: AuthenticationService,
    db: web::Data<Database>,
    path: web::Path<Uuid>,
    form: web::Json<HabitData>,
) -> HttpResponse {
    match Habit::edit(db.clone(), path.clone(), form.into_inner()).await {
        Ok(_) => match Habit::get_details(db.clone(), path.clone()).await {
            Ok(habit) => HttpResponse::Ok().json(habit),
            Err(err) => HttpResponse::InternalServerError().body(err),
        },
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

#[delete("/{habit_id}")]
async fn delete(
    _: AuthenticationService,
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let res = Habit::delete(db, path.clone()).await;

    match res {
        Ok(_) => HttpResponse::Ok().body("habit deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

#[put("/{habit_id}/archive")]
async fn archive(
    _: AuthenticationService,
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let res = Habit::archive(db, path.clone()).await;
    match res {
        Ok(_) => HttpResponse::Ok().body("habit archived"),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

#[put("/{habit_id}/clean")]
async fn clean_habit(
    _: AuthenticationService,
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let habit_id = path.clone();
    let res = Target::clean_habit(db.clone(), &habit_id).await;
    // TODO: refactor
    match res {
        Ok(_) => match Habit::get_by_id(db.clone(), habit_id.clone()).await {
            Ok(_) => match Habit::get_details(db.clone(), habit_id).await {
                Ok(habit) => HttpResponse::Ok().json(habit),
                Err(err) => HttpResponse::InternalServerError().body(err),
            },
            Err(err) => HttpResponse::InternalServerError().body(err),
        },
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

#[delete("/")]
async fn delete_habits(user: AuthenticationService, db: web::Data<Database>) -> HttpResponse {
    match Habit::delete_all_habits(db, user.0.id).await {
        Ok(_) => HttpResponse::Ok().body("habits deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}
