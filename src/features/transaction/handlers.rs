use crate::common::middlewares::auth::AuthenticationService;
use crate::features::account::models::Account;
use crate::features::transaction::models::{NewTransaction, Transaction, TransactionData};
use crate::repository::database::Database;
use actix_web::{get, post, web, HttpResponse, Scope};

pub fn routes() -> Scope {
    web::scope("/transaction")
        .service(create_transaction)
        .service(get_transactions)
}

#[get("")]
async fn get_transactions(user: AuthenticationService, db: web::Data<Database>) -> HttpResponse {
    let transactions = Transaction::get_all(db.clone(), user.0.id).await.unwrap();

    HttpResponse::Ok().json(transactions)
}

#[post("")]
async fn create_transaction(
    user: AuthenticationService,
    db: web::Data<Database>,
    form: web::Json<TransactionData>,
) -> HttpResponse {
    match Transaction::create(db.clone(), NewTransaction::create(&form.clone(), user.0.id)).await {
        Ok(_) => {
            match Account::update_amount(
                db.clone(),
                form.clone().account_id,
                form.clone().transaction_type,
                form.clone().amount,
            )
            .await
            {
                Ok(_) => HttpResponse::Ok().body("Transaction created"),
                Err(_) => HttpResponse::InternalServerError().body("Server error"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}
