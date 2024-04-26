use crate::features::user::models::User;
use crate::repository::database::Database;
use crate::schema::accounts;
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
#[diesel(table_name = accounts)]
pub struct Account {
    id: Uuid,
    user_id: Uuid,
    name: String,
    currency: String,     // "RUB", "USD", "EUR", "AMD"
    account_type: String, // "cash", "card", "deposit", "loan"
    amount: f64,
    created_date: DateTime<Utc>,
}

impl Account {
    pub async fn create(db: web::Data<Database>, account_data: NewAccount) -> Result<Uuid, String> {
        diesel::insert_into(accounts::table)
            .values(&account_data)
            .get_result::<Account>(&mut db.pool.get().unwrap())
            .map(|t| t.id)
            .map_err(|_| "Failed to create account".to_string())
    }

    pub async fn get_all(db: web::Data<Database>, user_id: Uuid) -> Result<Vec<Account>, String> {
        let mut accounts = accounts::table
            .filter(accounts::user_id.eq(user_id))
            .load::<Account>(&mut db.pool.get().unwrap())
            .expect("Error loading accounts");
        // TODO: include sort in query

        accounts.sort_by_key(|h| Reverse(h.created_date.clone()));

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
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount {
    user_id: Uuid,
    name: String,
    currency: String,
    account_type: String,
    amount: f64,
}

impl NewAccount {
    pub fn create(data: &AccountData, user_id: Uuid) -> Self {
        Self {
            user_id: user_id.clone(),
            name: data.name.clone(),
            currency: data.currency.clone(),
            account_type: data.account_type.clone(),
            amount: data.amount,
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
