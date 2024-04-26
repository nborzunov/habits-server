use std::str::FromStr;

use actix_web::{get, post, put, web, HttpRequest, HttpResponse, Scope};
use uuid::Uuid;

use crate::common::middlewares::auth::AuthenticationService;
use crate::common::services::crypto::Auth;
use crate::common::services::hashing::hashing;
use crate::features::achievement::models::Achievement;
use crate::features::category::models::Category;
use crate::features::user::models::{ChangePasswordData, NewUserData, UpdateUserData, User};
use crate::repository::database::Database;

pub fn routes() -> Scope {
    web::scope("/users")
        .service(create)
        .service(get)
        .service(update)
        .service(change_password)
}

#[post("/signup")]
async fn create(db: web::Data<Database>, form: web::Json<NewUserData>) -> HttpResponse {
    let res = User::create(db.clone(), form.clone()).await;

    match res {
        Ok(user) => {
            let token = hashing().generate_jwt(user.id).await.unwrap();

            tokio::spawn(Category::create_default(db.clone(), user.id));
            tokio::spawn(Achievement::create_default(
                db.clone(),
                user.id,
                "habits".to_string(),
            ));

            HttpResponse::Ok().json(Auth { token })
        }
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[get("/me")]
async fn get(db: web::Data<Database>, req: HttpRequest) -> HttpResponse {
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
            let user = User::get_by_id(db.clone(), Uuid::from_str(&v.claims.sub).unwrap())
                .await
                .unwrap();
            HttpResponse::Ok().json(&user)
        }
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}

#[put("/me")]
async fn update(
    user: AuthenticationService,
    db: web::Data<Database>,
    form: web::Json<UpdateUserData>,
) -> HttpResponse {
    let user_id = user.0.id;
    match User::update(db.clone(), user.0.id, form.into_inner()).await {
        Ok(_) => match User::get_by_id(db.clone(), user_id).await {
            Ok(u) => HttpResponse::Ok().json(&u),
            Err(err) => HttpResponse::InternalServerError().body(err),
        },
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

#[post("/me/change-password")]
async fn change_password(
    user: AuthenticationService,
    db: web::Data<Database>,
    form: web::Json<ChangePasswordData>,
) -> HttpResponse {
    match User::change_password(
        db.clone(),
        user.0.id,
        form.clone().current_password,
        form.clone().new_password,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().into(),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}
