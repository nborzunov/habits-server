use std::str::FromStr;

use actix_web::{get, post, put, web, HttpRequest, HttpResponse, Scope};
use mongodb::Client;

use crate::common::middlewares::auth::AuthenticationService;
use crate::common::services::crypto::Auth;
use crate::common::services::hashing::hashing;
use crate::users;
use crate::users::models::{ChangePasswordData, UpdateUserData, UserData, UserDetails};

pub fn routes() -> Scope {
    web::scope("/users")
        .service(create)
        .service(get)
        .service(update)
        .service(change_password)
}

#[post("/signup")]
pub async fn create(client: web::Data<Client>, form: web::Json<UserData>) -> HttpResponse {
    let res = users::repository::create(client.clone(), form.clone()).await;

    match res {
        Ok(user) => {
            let token = hashing().generate_jwt(user.id.unwrap()).await.unwrap();

            HttpResponse::Ok().json(Auth { token })
        }
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[get("/me")]
pub async fn get(client: web::Data<Client>, req: HttpRequest) -> HttpResponse {
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
            let user = users::repository::get_by_id(
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

#[put("/me")]
pub async fn update(
    user: AuthenticationService,
    client: web::Data<Client>,
    form: web::Json<UpdateUserData>,
) -> HttpResponse {
    let user_id = user.0.id.unwrap();
    match users::repository::update(client.clone(), user.0.id.unwrap(), form.into_inner()).await {
        Ok(_) => match users::repository::get_by_id(client.clone(), user_id).await {
            Ok(u) => HttpResponse::Ok().json(UserDetails::parse(&u)),
            Err(err) => HttpResponse::InternalServerError().body(err),
        },
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

#[post("/me/change-password")]
pub async fn change_password(
    user: AuthenticationService,
    client: web::Data<Client>,
    form: web::Json<ChangePasswordData>,
) -> HttpResponse {
    match users::repository::change_password(
        client.clone(),
        user.0.id.unwrap(),
        form.clone().current_password,
        form.clone().new_password,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().into(),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}
