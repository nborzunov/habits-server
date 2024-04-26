use crate::features::user::models::User;
use crate::repository::database::Database;
use crate::schema::categories;
use actix_web::web;
use chrono::DateTime;
use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use uuid::Uuid;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Selectable,
    Identifiable,
    Associations,
    Queryable,
    PartialEq,
)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = categories)]
pub struct Category {
    pub id: Uuid,
    user_id: Uuid,
    pub category_type: String, // "income", "expense"
    name: String,
    icon: String,
    color: String,
    is_default: bool,
    created_date: DateTime<Utc>,
    modified_date: Option<DateTime<Utc>>,
}

impl Category {
    pub async fn create(
        db: web::Data<Database>,
        category_data: NewCategory,
    ) -> Result<Uuid, String> {
        diesel::insert_into(categories::table)
            .values(&category_data)
            .get_result::<Category>(&mut db.pool.get().unwrap())
            .map(|t| t.id)
            .map_err(|_| "Failed to create category".to_string())
    }

    pub async fn get_all_raw(
        db: web::Data<Database>,
        user_id: Uuid,
    ) -> Result<Vec<Category>, String> {
        let mut categories = categories::table
            .filter(categories::user_id.eq(user_id))
            .load::<Category>(&mut db.pool.get().unwrap())
            .expect("Error loading categories");
        // TODO: include sort in query

        categories.sort_by_key(|h| Reverse(h.created_date.clone()));

        return Ok(categories);
    }

    pub async fn get_all(
        client: web::Data<Database>,
        user_id: Uuid,
    ) -> Result<CategoriesResult, String> {
        let categories = Self::get_all_raw(client, user_id).await?;

        let income_categories: Vec<Category> = categories
            .iter()
            .filter(|c| c.category_type == "income")
            .cloned()
            .collect();

        let expense_categories: Vec<Category> = categories
            .iter()
            .filter(|c| c.category_type == "expense")
            .cloned()
            .collect();

        Ok(CategoriesResult {
            income: income_categories,
            expense: expense_categories,
        })
    }

    pub async fn get_by_id(db: web::Data<Database>, id: Uuid) -> Result<Category, String> {
        categories::table
            .filter(categories::id.eq(id))
            .first::<Category>(&mut db.pool.get().unwrap())
            .map_err(|_| "Category not found".to_string())
    }

    pub async fn delete(db: web::Data<Database>, id: Uuid) -> Result<Uuid, String> {
        let category = categories::table.filter(categories::id.eq(id));
        diesel::delete(category)
            .execute(&mut db.pool.get().unwrap())
            .map(|_| id)
            .map_err(|_| "Error deleting category".to_string())
    }

    pub async fn create_default(db: web::Data<Database>, user_id: Uuid) -> Result<(), String> {
        let income_categories = vec![
            ("salary", "orange"),
            ("freelance", "yellow"),
            ("rental", "green"),
            ("bonus", "teal"),
        ];

        let expense_categories = vec![
            ("groceries", "orange"),
            ("dining", "yellow"),
            ("home", "green"),
            ("utilities", "teal"),
            ("bills", "blue"),
            ("taxes", "cyan"),
            ("transportation", "purple"),
            ("entertainment", "pink"),
            ("healthcare", "red"),
            ("education", "orange"),
        ];

        for (name, color) in income_categories {
            Self::create(
                db.clone(),
                NewCategory::create(
                    &CategoryData {
                        category_type: "income".to_string(),
                        name: name.to_string(),
                        color: color.to_string(),
                        icon: name.to_string(),
                        is_default: true,
                    },
                    user_id.clone(),
                ),
            )
            .await?;
        }

        for (name, color) in expense_categories {
            Self::create(
                db.clone(),
                NewCategory::create(
                    &CategoryData {
                        category_type: "expense".to_string(),
                        name: name.to_string(),
                        color: color.to_string(),
                        icon: name.to_string(),
                        is_default: true,
                    },
                    user_id.clone(),
                ),
            )
            .await?;
        }

        return Ok(());
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable)]
#[diesel(table_name = categories)]
pub struct NewCategory {
    pub user_id: Uuid,
    pub category_type: String,
    pub name: String,
    pub color: String,
    pub icon: String,
    pub is_default: bool,
}

impl NewCategory {
    pub fn create(data: &CategoryData, user_id: Uuid) -> Self {
        Self {
            user_id: user_id.clone(),
            category_type: data.category_type.clone(),
            name: data.name.clone(),
            color: data.color.clone(),
            icon: data.icon.clone(),
            is_default: data.is_default,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = categories)]
pub struct CategoryData {
    pub category_type: String,
    pub name: String,
    pub color: String,
    pub icon: String,
    pub is_default: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoriesResult {
    pub income: Vec<Category>,
    pub expense: Vec<Category>,
}
