use crate::features::user::models::User;
use crate::repository::database::Database;
use crate::schema::accounts;
use actix_web::web;
use chrono::DateTime;
use chrono::Utc;
use diesel::dsl::max;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
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
#[diesel(table_name = accounts)]
pub struct Account {
    id: Uuid,
    user_id: Uuid,
    name: String,
    currency: String,     // "RUB", "USD", "EUR", "AMD"
    account_type: String, // "cash", "card", "deposit", "loan"
    amount: f64,
    created_date: DateTime<Utc>,
    a_order: i32,
}

impl Account {
    pub async fn create(db: web::Data<Database>, account_data: AccountData, user_id: Uuid) -> Result<Uuid, String> {
        let a_order: Option<i32> = accounts::table
            .select(max(accounts::a_order))
            .first(&mut db.pool.get().unwrap())
            .unwrap();

        let next_order_number = match a_order {
            Some(max_order) => max_order + 1,
            None => 0, // Default to 1 if there are no existing orders
        };

        diesel::insert_into(accounts::table)
            .values(NewAccount::create(&account_data, user_id.clone(), next_order_number))
            .get_result::<Account>(&mut db.pool.get().unwrap())
            .map(|t| t.id)
            .map_err(|_| "Failed to create account".to_string())
    }

    pub async fn get_all(db: web::Data<Database>, user_id: Uuid) -> Result<Vec<Account>, String> {
        let accounts = accounts::table
            .filter(accounts::user_id.eq(user_id))
            .order(accounts::a_order.asc())
            .load::<Account>(&mut db.pool.get().unwrap())
            .expect("Error loading accounts");

        return Ok(accounts);
    }

    pub async fn update_amount(
        db: web::Data<Database>,
        id: Uuid,
        transaction_type: String,
        amount: f64,
    ) -> Result<(), String> {
        diesel::update(accounts::table)
            .filter(accounts::id.eq(id.clone()))
            .set(accounts::amount.eq(match transaction_type.as_str() {
                "income" => accounts::amount + amount,
                "expense" => accounts::amount + -amount,
                _ => return Err("Invalid transaction type".to_string()),
            }))
            .execute(&mut db.pool.get().unwrap())
            .map(|_| ())
            .map_err(|_| "Failed to update account amount".to_string())
    }

    pub async fn update(db: web::Data<Database>, id: Uuid, data: UpdateAccountData) -> Result<(), String> {
        diesel::update(accounts::table)
            .filter(accounts::id.eq(id.clone()))
            .set(data)
            .execute(&mut db.pool.get().unwrap())
            .map(|_| ())
            .map_err(|_| "Failed to update account".to_string())
    }

    pub async fn reorder(db: web::Data<Database>, data: Vec<ReorderAccountsData>) -> Result<(), String> {
        for d in data {
            let _ = diesel::update(accounts::table)
                .filter(accounts::id.eq(d.id))
                .set(accounts::a_order.eq(d.a_order))
                .execute(&mut db.pool.get().unwrap())
                .map(|_| ())
                .map_err(|_| "Failed to update account".to_string());
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = accounts)]
pub struct NewAccount {
    user_id: Uuid,
    name: String,
    currency: String,
    account_type: String,
    amount: f64,
    a_order: i32,
}

impl NewAccount {
    pub fn create(data: &AccountData, user_id: Uuid, a_order: i32) -> Self {
        Self {
            user_id: user_id.clone(),
            name: data.name.clone(),
            currency: data.currency.clone(),
            account_type: data.account_type.clone(),
            amount: data.amount,
            a_order: a_order,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = accounts)]
pub struct AccountData {
    name: String,
    currency: String,
    account_type: String,
    amount: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = accounts)]
pub struct UpdateAccountData {
    name: Option<String>,
    currency: Option<String>,
    account_type: Option<String>,
    amount: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = accounts)]
pub struct ReorderAccountsData {
    id: Uuid,
    a_order: i32,
}
