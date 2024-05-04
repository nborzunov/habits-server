use crate::features::account::models::Account;
use crate::features::category::models::Category;
use crate::features::user::models::User;
use crate::repository::database::Database;
use crate::schema::{accounts, categories, transactions};
use actix_web::web;
use chrono::DateTime;
use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Identifiable,
    Associations,
    PartialEq,
)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Account, foreign_key = account_id))]
#[diesel(belongs_to(Category, foreign_key = category_id))]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub account_id: Uuid,
    pub category_id: Uuid,
    pub transaction_type: String, // "income",  "expense"
    pub note: Option<String>,
    pub amount: f64,
    pub created_date: DateTime<Utc>,
    pub archived: bool,
    pub deleted: bool,
}

impl Transaction {
    pub async fn create(
        db: web::Data<Database>,
        transaction_data: NewTransaction,
    ) -> Result<Uuid, String> {
        diesel::insert_into(transactions::table)
            .values(&transaction_data)
            .get_result::<Transaction>(&mut db.pool.get().unwrap())
            .map(|t| t.id)
            .map_err(|err| err.to_string())
    }

    pub async fn get_all(
        db: web::Data<Database>,
        user_id: Uuid,
    ) -> Result<Vec<TransactionDetails>, String> {
        transactions::table
            .inner_join(accounts::table.on(accounts::id.eq(transactions::account_id)))
            .inner_join(categories::table.on(categories::id.eq(transactions::category_id)))
            .filter(transactions::user_id.eq(user_id))
            .select((
                Transaction::as_select(),
                Account::as_select(),
                Category::as_select(),
            ))
            .order_by(transactions::created_date.desc())
            .load::<(Transaction, Account, Category)>(&mut db.pool.get().unwrap())
            .map(|transactions| {
                transactions
                    .into_iter()
                    .map(|t| TransactionDetails::from(t))
                    .collect::<Vec<TransactionDetails>>()
            })
            .map_err(|_| "Error loading transactions".to_string())
    }

    pub async fn delete_by_category(
        db: web::Data<Database>,
        category_id: Uuid,
    ) -> Result<(), String> {
        let transaction = transactions::table.filter(transactions::category_id.eq(category_id));
        diesel::delete(transaction)
            .execute(&mut db.pool.get().unwrap())
            .map(|_| ())
            .map_err(|_| "Error deleting transaction".to_string())
    }

    pub async fn delete_by_account(
        db: web::Data<Database>,
        account_id: Uuid,
    ) -> Result<(), String> {
        let transaction = transactions::table.filter(transactions::account_id.eq(account_id));
        diesel::delete(transaction)
            .execute(&mut db.pool.get().unwrap())
            .map(|_| ())
            .map_err(|_| "Error deleting transaction".to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    user_id: Uuid,
    pub account_id: Uuid,
    pub category_id: Uuid,
    pub transaction_type: String,
    pub note: Option<String>,
    pub amount: f64,
    pub created_date: DateTime<Utc>,
}

impl NewTransaction {
    pub fn create(data: &TransactionData, user_id: Uuid) -> Self {
        Self {
            user_id: user_id.clone(),
            account_id: data.account_id.clone(),
            category_id: data.category_id.clone(),
            transaction_type: data.transaction_type.clone(),
            note: data.note.clone(),
            amount: data.amount,
            created_date: data.created_date,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable)]
#[diesel(table_name = transactions)]
pub struct TransactionData {
    pub account_id: Uuid,
    category_id: Uuid,
    pub transaction_type: String,
    note: Option<String>,
    pub amount: f64,
    created_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionDetails {
    pub id: Uuid,
    pub user_id: Uuid,
    pub account_id: Uuid,
    pub category_id: Uuid,
    pub transaction_type: String,
    pub note: Option<String>,
    pub amount: f64,
    pub created_date: DateTime<Utc>,
    pub archived: bool,
    pub deleted: bool,
    pub account: Account,
    pub category: Category,
}

impl From<(Transaction, Account, Category)> for TransactionDetails {
    fn from(t: (Transaction, Account, Category)) -> Self {
        Self {
            id: t.0.id,
            user_id: t.0.user_id,
            account_id: t.0.account_id,
            category_id: t.0.category_id,
            transaction_type: t.0.transaction_type,
            note: t.0.note,
            amount: t.0.amount,
            created_date: t.0.created_date,
            archived: t.0.archived,
            deleted: t.0.deleted,
            account: t.1,
            category: t.2,
        }
    }
}
// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Transfer {
//     #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
//     id: Uuid,
//     user_id: Uuid,
//     from_account_id: Uuid,
//     to_account_id: Uuid,
//     note: String,
//     amount: i32,
//     created_date: DateTime<Utc>,
//     archived: bool,
//     deleted: bool,
// }
