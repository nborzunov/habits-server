use crate::common::middlewares::auth::AuthenticationService;
use crate::features::account::models::ReorderAccountsData;
use crate::{
    features::account::models::{Account, AccountData},
    repository::database::Database,
};
use actix_web::{get, post, web, HttpResponse, Scope};

pub fn routes() -> Scope {
    web::scope("/account")
        .service(create_account)
        .service(get_accounts)
        .service(reorder_accounts)
}

#[post("")]
async fn create_account(
    user: AuthenticationService,
    db: web::Data<Database>,
    form: web::Json<AccountData>,
) -> HttpResponse {
    match Account::create(
        db.clone(),
        form.into_inner(),
        user.0.id,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().json(Account::get_all(db.clone(), user.0.id).await.unwrap()),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

#[get("")]
async fn get_accounts(user: AuthenticationService, db: web::Data<Database>) -> HttpResponse {
    let accounts = Account::get_all(db.clone(), user.0.id).await.unwrap();

    HttpResponse::Ok().json(accounts)
}

#[post("/reorder")]
async fn reorder_accounts(
    user: AuthenticationService,
    db: web::Data<Database>,
    form: web::Json<Vec<ReorderAccountsData>>,
) -> HttpResponse {
    match Account::reorder(db.clone(), form.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(Account::get_all(db.clone(), user.0.id).await.unwrap()),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}