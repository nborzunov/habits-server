use std::str::FromStr;

use actix_web::{get, post, web, HttpRequest, HttpResponse, Scope};
use mongodb::Client;

use crate::models::user::{UserData, UserDetails};
use crate::repository;
use crate::services::crypto::Auth;
use crate::services::hashing::hashing;

pub fn routes() -> Scope {
    web::scope("/users").service(create).service(get_user)
}

#[post("/signup")]
pub async fn create(client: web::Data<Client>, form: web::Json<UserData>) -> HttpResponse {
    let res = repository::users::create(client.clone(), form.clone()).await;

    match res {
        Ok(user) => {
            let token = hashing().generate_jwt(user.id.unwrap()).await.unwrap();

            HttpResponse::Ok().json(Auth { token })
        }
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

#[get("/me")]
pub async fn get_user(client: web::Data<Client>, req: HttpRequest) -> HttpResponse {
    let token = req
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .replace("Bearer ", "");

    let verified = hashing().verify_jwt(token.to_string()).await;
    match verified {
        Ok(v) => {
            let user = repository::users::get_by_id(
                client.clone(),
                mongodb::bson::oid::ObjectId::from_str(&v.claims.sub.clone()).unwrap(),
            )
            .await
            .unwrap();
            HttpResponse::Ok().json(UserDetails::parse(&user))
        }
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}
