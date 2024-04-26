use crate::common::models::errors::FormError;
use crate::common::services::crypto::Auth;
use crate::common::services::hashing::hashing;
use crate::features::auth::models::LoginData;
use crate::features::user::models::User;
use crate::repository::database::Database;

use actix_web::web::Data;
use actix_web::{post, web, HttpResponse, Scope};

pub fn routes() -> Scope {
    web::scope("/auth").service(login)
}

#[post("/")]
async fn login(db: Data<Database>, form: web::Json<LoginData>) -> HttpResponse {
    let username = form.username.clone();
    let password = form.password.clone();

    let user = match User::get_by_username(db.clone(), username.to_string()).await {
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
        let token = hashing().generate_jwt(user.id).await.unwrap();

        // TODO: make access and refresh tokens
        HttpResponse::Ok().json(Auth { token })
    } else {
        HttpResponse::BadRequest().json(FormError {
            field: "password",
            message: "profile:password.errors.invalid",
        })
    }
}
