pub mod transactions {
    use crate::finance::models::accounts::AccountDetails;
    use crate::finance::models::categories::CategoryDetails;
    use chrono::{DateTime, Utc};
    use mongodb::bson::oid::ObjectId;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum TransactionType {
        Income,
        Expense,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Transaction {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        id: Option<ObjectId>,
        user_id: ObjectId,
        pub account_id: ObjectId,
        pub category_id: ObjectId,
        transaction_type: TransactionType,
        note: String,
        amount: f64,
        created_date: DateTime<Utc>,
        archived: bool,
        deleted: bool,
    }

    impl Transaction {
        pub fn new(data: &TransactionData, user_id: ObjectId) -> Self {
            Transaction {
                id: None,
                user_id: user_id.clone(),
                account_id: data.account_id.clone(),
                category_id: data.category_id.clone(),
                transaction_type: data.transaction_type.clone(),
                note: data.note.clone(),
                amount: data.amount,
                created_date: data.date,
                archived: false,
                deleted: false,
            }
        }

        pub fn get_details(
            &self,
            account: AccountDetails,
            category: CategoryDetails,
        ) -> TransactionDetails {
            TransactionDetails {
                id: self.id.clone().unwrap().to_string(),
                account,
                category,
                transaction_type: self.transaction_type.clone(),
                note: self.note.clone(),
                amount: self.amount,
                date: self.created_date,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct TransactionDetails {
        pub id: String,
        pub account: AccountDetails,
        pub category: CategoryDetails,
        pub transaction_type: TransactionType,
        pub note: String,
        pub amount: f64,
        pub date: DateTime<Utc>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct TransactionData {
        pub account_id: ObjectId,
        category_id: ObjectId,
        pub transaction_type: TransactionType,
        note: String,
        pub amount: f64,
        date: DateTime<Utc>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Transfer {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        id: Option<ObjectId>,
        user_id: ObjectId,
        from_account_id: ObjectId,
        to_account_id: ObjectId,
        note: String,
        amount: i32,
        created_date: DateTime<Utc>,
        archived: bool,
        deleted: bool,
    }
}

pub mod accounts {
    use mongodb::bson::oid::ObjectId;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum Currency {
        RUB,
        USD,
        EUR,
        AMD,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum AccountType {
        Cash,
        Card,
        Deposit,
        Loan,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Account {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        id: Option<ObjectId>,
        user_id: ObjectId,
        name: String,
        currency: Currency,
        account_type: AccountType,
        amount: f64,
    }

    impl Account {
        pub fn new(data: &AccountData, user_id: ObjectId) -> Self {
            Account {
                id: None,
                user_id: user_id.clone(),
                name: data.name.clone(),
                currency: data.currency.clone(),
                account_type: data.account_type.clone(),
                amount: data.amount,
            }
        }

        pub fn get_details(&self) -> AccountDetails {
            AccountDetails {
                id: self.id.clone().unwrap().to_string(),
                name: self.name.clone(),
                currency: self.currency.clone(),
                account_type: self.account_type.clone(),
                amount: self.amount,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct AccountData {
        name: String,
        currency: Currency,
        account_type: AccountType,
        amount: f64,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct AccountDetails {
        pub id: String,
        pub name: String,
        currency: Currency,
        account_type: AccountType,
        amount: f64,
    }
}

pub mod categories {
    use crate::finance::models::transactions::TransactionType;
    use chrono::{DateTime, Utc};
    use mongodb::bson::oid::ObjectId;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Category {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        pub id: Option<ObjectId>,
        user_id: ObjectId,
        pub category_type: TransactionType,
        name: String,
        icon: String,
        color: String,
        default: bool,
        created_date: DateTime<Utc>,
        modified_date: Option<DateTime<Utc>>,
    }

    impl Category {
        pub fn new(data: &CategoryData, user_id: ObjectId) -> Self {
            Category {
                id: None,
                user_id: user_id.clone(),
                category_type: data.category_type.clone(),
                name: data.name.clone(),
                icon: data.icon.clone(),
                color: data.color.clone(),
                default: data.default,
                created_date: Utc::now(),
                modified_date: None,
            }
        }

        pub fn get_details(&self) -> CategoryDetails {
            CategoryDetails {
                id: self.id.clone().unwrap().to_string(),
                category_type: self.category_type.clone(),
                name: self.name.clone(),
                color: self.color.clone(),
                icon: self.icon.clone(),
                default: self.default,
            }
        }
    }
    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct CategoryDetails {
        pub id: String,
        pub category_type: TransactionType,
        pub name: String,
        pub color: String,
        pub icon: String,
        pub default: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct CategoryData {
        pub category_type: TransactionType,
        pub name: String,
        pub color: String,
        pub icon: String,
        pub default: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct CategoriesResult {
        pub income: Vec<CategoryDetails>,
        pub expense: Vec<CategoryDetails>,
    }
}
