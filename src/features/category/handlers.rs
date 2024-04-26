use crate::features::category::models::{Category, CategoryData, NewCategory};
use crate::features::transaction::models::Transaction;
use crate::{common::middlewares::auth::AuthenticationService, repository::database::Database};
use actix_web::{delete, get, post, web, HttpResponse, Scope};
use uuid::Uuid;

pub fn routes() -> Scope {
    web::scope("/category")
        .service(create_category)
        .service(get_categories)
        .service(get_category)
        .service(delete_category)
}

#[post("")]
async fn create_category(
    user: AuthenticationService,
    db: web::Data<Database>,
    form: web::Json<CategoryData>,
) -> HttpResponse {
    match Category::create(
        db.clone(),
        NewCategory::create(&form.into_inner(), user.0.id),
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().body("Category created"),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

#[get("")]
async fn get_categories(user: AuthenticationService, db: web::Data<Database>) -> HttpResponse {
    let categories = Category::get_all(db.clone(), user.0.id).await;

    match categories {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

#[get("{category_id}")]
async fn get_category(
    _: AuthenticationService,
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let categories = Category::get_by_id(db.clone(), path.into_inner()).await;

    match categories {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}

#[delete("{category_id}")]
async fn delete_category(
    _: AuthenticationService,
    db: web::Data<Database>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let category_id = Category::delete(db.clone(), path.clone()).await.unwrap();

    match Transaction::delete_by_category(db.clone(), category_id).await {
        Ok(_) => HttpResponse::Ok().body("Category deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Server error"),
    }
}
