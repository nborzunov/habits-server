use actix_web::{post, web, HttpResponse, Scope};
use mongodb::Client;

use crate::middlewares::auth::AuthenticationService;
use crate::models::targets::{Target, TargetData};
use crate::repository;

pub fn routes() -> Scope {
    web::scope("/targets").service(create)
}

#[post("/")]
pub async fn create(
    user: AuthenticationService,
    client: web::Data<Client>,
    form: web::Json<TargetData>,
) -> HttpResponse {
    match repository::targets::create(
        client.clone(),
        user.0.id.unwrap(),
        Target::new(&form.clone()),
    )
    .await
    {
        Ok(_) => match repository::habits::get_details(client.clone(), form.habit_id).await {
            Ok(habit) => HttpResponse::Ok().json(habit),
            Err(err) => HttpResponse::InternalServerError().body(err),
        },
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}
