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
    let res = repository::targets::create(
        client.clone(),
        user.0.id.unwrap(),
        Target::new(&form.clone()),
    )
    .await;

    match res {
        Ok(_) => HttpResponse::Ok()
            .json(repository::habits::get_details(client.clone(), form.habit_id).await),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}
