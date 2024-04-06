use crate::auth::models::LoginData;
use crate::common::models::errors::FormError;
use crate::common::services::crypto::Auth;
use crate::common::services::hashing::hashing;
use crate::users;
use actix_web::web::Data;
use actix_web::{post, web, HttpResponse, Scope};
use mongodb::Client;

pub fn routes() -> Scope {
    web::scope("/auth").service(login)
}

#[post("/")]
pub async fn login(client: Data<Client>, form: web::Json<LoginData>) -> HttpResponse {
    println!("Login request: {:?}", form);
    let username = form.username.clone();
    let password = form.password.clone();

    let user = match users::repository::get_by_username(client.clone(), username.to_string()).await
    {
        Ok(user) => user,
        Err(_) => {
            return HttpResponse::BadRequest().json(FormError {
                field: "username",
                message: "profile:username.errors.notFound",
            });
        }
    };

    let valid = hashing()
        .verify_password(&password, &user.password_hash)
        .await
        .unwrap();

    if valid {
        let token = hashing().generate_jwt(user.id.unwrap()).await.unwrap();
        // TODO: make access and refresh tokens
        HttpResponse::Ok().json(Auth { token })
    } else {
        HttpResponse::BadRequest().json(FormError {
            field: "password",
            message: "profile:password.errors.invalid",
        })
    }
}
