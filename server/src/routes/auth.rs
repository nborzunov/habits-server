use actix_web::web::Data;
use actix_web::{post, web, HttpResponse, Scope};
use mongodb::Client;

use crate::models::auth::LoginData;
use crate::repository;
use crate::services::crypto::Auth;
use crate::services::hashing::hashing;

pub fn routes() -> Scope {
    web::scope("/auth").service(login)
}

#[post("/")]
pub async fn login(client: Data<Client>, form: web::Json<LoginData>) -> HttpResponse {
    let username = form.username.clone();
    let password = form.password.clone();

    let user = repository::users::get_by_username(client.clone(), username.to_string())
        .await
        .unwrap();

    let valid = hashing()
        .verify_password(&password, &user.password_hash)
        .await
        .unwrap();

    if valid {
        let token = hashing().generate_jwt(user.id.unwrap()).await.unwrap();
        // TODO: make access and refresh tokens
        HttpResponse::Ok().json(Auth { token })
    } else {
        HttpResponse::Unauthorized().finish()
    }
}
