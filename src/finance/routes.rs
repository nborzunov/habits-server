use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("/finance")
        .service(transactions::create_transaction)
        .service(transactions::get_transactions)
        .service(accounts::create_account)
        .service(accounts::get_accounts)
        .service(categories::create_category)
        .service(categories::get_categories)
        .service(categories::get_category)
        .service(categories::delete_category)
}

pub mod transactions {
    use crate::common::middlewares::auth::AuthenticationService;
    use crate::finance;
    use crate::finance::models::transactions::{Transaction, TransactionData};
    use actix_web::{get, post, web, HttpResponse};
    use mongodb::Client;

    #[get("/transaction")]
    pub async fn get_transactions(
        user: AuthenticationService,
        client: web::Data<Client>,
    ) -> HttpResponse {
        let accounts =
            finance::repository::transactions::get_all(client.clone(), user.0.id.unwrap())
                .await
                .unwrap();

        HttpResponse::Ok().json(accounts)
    }

    #[post("/transaction")]
    pub async fn create_transaction(
        user: AuthenticationService,
        client: web::Data<Client>,
        form: web::Json<TransactionData>,
    ) -> HttpResponse {
        match finance::repository::transactions::create(
            client.clone(),
            Transaction::new(&form.clone(), user.0.id.unwrap()),
        )
        .await
        {
            Ok(_) => {
                match finance::repository::accounts::update_amount(
                    client.clone(),
                    user.0.id.unwrap(),
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
}

pub mod accounts {
    use crate::common::middlewares::auth::AuthenticationService;
    use crate::finance;
    use crate::finance::models::accounts::{Account, AccountData};
    use actix_web::{get, post, web, HttpResponse};
    use mongodb::Client;

    #[post("/account")]
    pub async fn create_account(
        user: AuthenticationService,
        client: web::Data<Client>,
        form: web::Json<AccountData>,
    ) -> HttpResponse {
        match finance::repository::accounts::create(
            client.clone(),
            Account::new(&form.into_inner(), user.0.id.unwrap()),
        )
        .await
        {
            Ok(_) => HttpResponse::Ok().json(
                finance::repository::accounts::get_all(client.clone(), user.0.id.unwrap())
                    .await
                    .unwrap(),
            ),
            Err(_) => HttpResponse::InternalServerError().body("Server error"),
        }
    }

    #[get("/account")]
    pub async fn get_accounts(
        user: AuthenticationService,
        client: web::Data<Client>,
    ) -> HttpResponse {
        let accounts = finance::repository::accounts::get_all(client.clone(), user.0.id.unwrap())
            .await
            .unwrap();

        HttpResponse::Ok().json(accounts)
    }
}

pub mod categories {
    use std::str::FromStr;

    use crate::common::middlewares::auth::AuthenticationService;
    use crate::finance;
    use crate::finance::models::categories::{Category, CategoryData};
    use actix_web::{delete, get, post, web, HttpResponse};
    use mongodb::bson::oid::ObjectId;
    use mongodb::Client;

    #[post("/category")]
    pub async fn create_category(
        user: AuthenticationService,
        client: web::Data<Client>,
        form: web::Json<CategoryData>,
    ) -> HttpResponse {
        match finance::repository::categories::create(
            client.clone(),
            Category::new(&form.into_inner(), user.0.id.unwrap()),
        )
        .await
        {
            Ok(_) => HttpResponse::Ok().body("Category created"),
            Err(_) => HttpResponse::InternalServerError().body("Server error"),
        }
    }

    #[get("/category")]
    pub async fn get_categories(
        user: AuthenticationService,
        client: web::Data<Client>,
    ) -> HttpResponse {
        let categories =
            finance::repository::categories::get_all(client.clone(), user.0.id.unwrap()).await;

        match categories {
            Ok(categories) => HttpResponse::Ok().json(categories),
            Err(_) => HttpResponse::InternalServerError().body("Server error"),
        }
    }

    #[get("/category/{category_id}")]
    pub async fn get_category(
        user: AuthenticationService,
        client: web::Data<Client>,
        path: web::Path<String>,
    ) -> HttpResponse {
        let categories = finance::repository::categories::get(
            client.clone(),
            user.0.id.unwrap(),
            path.into_inner(),
        )
        .await;

        match categories {
            Ok(categories) => HttpResponse::Ok().json(categories),
            Err(_) => HttpResponse::InternalServerError().body("Server error"),
        }
    }

    #[delete("/category/{category_id}")]
    pub async fn delete_category(
        user: AuthenticationService,
        client: web::Data<Client>,
        path: web::Path<String>,
    ) -> HttpResponse {
        match finance::repository::categories::delete(
            client.clone(),
            user.0.id.unwrap(),
            ObjectId::from_str(&path.into_inner()).unwrap(),
        )
        .await
        {
            Ok(_) => HttpResponse::Ok().body("Category deleted"),
            Err(_) => HttpResponse::InternalServerError().body("Server error"),
        }
    }
}
