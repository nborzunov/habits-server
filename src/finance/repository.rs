pub mod transactions {
    use crate::finance::models::accounts::AccountDetails;
    use crate::finance::models::categories::CategoryDetails;
    use crate::finance::models::transactions::{Transaction, TransactionDetails};
    use crate::{finance, DB_NAME};
    use actix_web::web;
    use futures::TryStreamExt;
    use mongodb::bson::doc;
    use mongodb::bson::oid::ObjectId;
    use mongodb::Client;
    use std::collections::HashMap;
    use std::str::FromStr;

    const COLL_NAME: &str = "transactions";

    pub async fn create(
        client: web::Data<Client>,
        transaction: Transaction,
    ) -> Result<ObjectId, String> {
        // TODO: validate account id, user id, category id
        client
            .database(&DB_NAME)
            .collection(COLL_NAME)
            .insert_one(transaction, None)
            .await
            .map_or_else(
                |_| Err("Failed to create transaction".to_string()),
                |result| Ok(result.inserted_id.as_object_id().unwrap().clone()),
            )
    }

    pub async fn get_all(
        client: web::Data<Client>,
        user_id: ObjectId,
    ) -> Result<Vec<TransactionDetails>, String> {
        let docs = client
            .database(&DB_NAME)
            .collection::<Transaction>(COLL_NAME)
            .find(
                doc! {
                    "userId": &user_id,
                },
                None,
            )
            .await;

        let accounts = finance::repository::accounts::get_all(client.clone(), user_id.clone())
            .await
            .unwrap();
        let accounts_map = accounts
            .iter()
            .map(|a| (ObjectId::from_str(a.id.as_str()).unwrap(), a.clone()))
            .collect::<HashMap<ObjectId, AccountDetails>>();
        let categories =
            finance::repository::categories::get_all_raw(client.clone(), user_id.clone())
                .await
                .unwrap();
        let categories_map = categories
            .iter()
            .map(|c| (c.id.unwrap().clone(), c.clone().get_details()))
            .collect::<HashMap<ObjectId, CategoryDetails>>();

        return match docs {
            Ok(cursor) => Ok(cursor
                .try_collect::<Vec<Transaction>>()
                .await
                .map(|t| {
                    let mut transactions: Vec<TransactionDetails> = t
                        .iter()
                        .map(|t| {
                            let account = accounts_map.get(&t.account_id).unwrap();
                            println!("categories_map: {:?}", categories_map);
                            let category = categories_map.get(&t.category_id).unwrap();
                            t.get_details(account.clone(), category.clone())
                        })
                        .collect();

                    // sort from newest to oldest
                    transactions.sort_by(|a, b| b.date.cmp(&a.date));
                    return transactions;
                })
                .map_err(|_| "Failed to collect transactions".to_string())?),
            Err(_) => Err("Failed to get transactions".to_string()),
        };
    }
}

pub mod accounts {
    use crate::finance::models::accounts::{Account, AccountDetails};
    use crate::finance::models::transactions::TransactionType;
    use crate::DB_NAME;
    use actix_web::web;
    use futures::TryStreamExt;
    use mongodb::bson::doc;
    use mongodb::bson::oid::ObjectId;
    use mongodb::Client;

    const COLL_NAME: &str = "accounts";

    pub async fn create(client: web::Data<Client>, account: Account) -> Result<ObjectId, String> {
        client
            .database(&DB_NAME)
            .collection(COLL_NAME)
            .insert_one(account, None)
            .await
            .map_or_else(
                |_| Err("Failed to create account".to_string()),
                |result| Ok(result.inserted_id.as_object_id().unwrap().clone()),
            )
    }

    pub async fn get_all(
        client: web::Data<Client>,
        user_id: ObjectId,
    ) -> Result<Vec<AccountDetails>, String> {
        let docs = client
            .database(&DB_NAME)
            .collection::<Account>(COLL_NAME)
            .find(
                doc! {
                    "userId": &user_id,
                },
                None,
            )
            .await;

        return match docs {
            Ok(cursor) => Ok(cursor
                .try_collect::<Vec<Account>>()
                .await
                .map(|a| a.iter().map(|a| a.get_details()).collect())
                .map_err(|_| "Failed to collect accounts".to_string())?),
            Err(_) => Err("Failed to get accounts".to_string()),
        };
    }

    pub async fn update_amount(
        client: web::Data<Client>,
        user_id: ObjectId,
        account_id: ObjectId,
        transaction_type: TransactionType,
        amount: f64,
    ) -> Result<(), String> {
        let collection = client.database(&DB_NAME).collection::<Account>(COLL_NAME);
        let filter = doc! {
            "_id": &account_id,
            "userId": &user_id,
        };
        let update = match transaction_type {
            TransactionType::Income => doc! {
                "$inc": {
                    "amount": amount,
                }
            },
            TransactionType::Expense => doc! {
                "$inc": {
                    "amount": -amount,
                }
            },
        };

        collection
            .update_one(filter, update, None)
            .await
            .map_or_else(
                |_| Err("Failed to update account amount".to_string()),
                |_| Ok(()),
            )
    }
}

pub mod categories {
    use crate::finance::models::categories::{
        CategoriesResult, Category, CategoryData, CategoryDetails,
    };
    use crate::finance::models::transactions::TransactionType;
    use crate::DB_NAME;
    use actix_web::web;
    use futures::TryStreamExt;
    use mongodb::bson::doc;
    use mongodb::bson::oid::ObjectId;
    use mongodb::Client;

    const COLL_NAME: &str = "categories";

    pub async fn create(client: web::Data<Client>, category: Category) -> Result<ObjectId, String> {
        client
            .database(&DB_NAME)
            .collection(COLL_NAME)
            .insert_one(category, None)
            .await
            .map_or_else(
                |_| Err("Failed to create category".to_string()),
                |result| Ok(result.inserted_id.as_object_id().unwrap().clone()),
            )
    }

    pub async fn get_all_raw(
        client: web::Data<Client>,
        user_id: ObjectId,
    ) -> Result<Vec<Category>, String> {
        let docs = client
            .database(&DB_NAME)
            .collection::<Category>(COLL_NAME)
            .find(
                doc! {
                    "userId": &user_id,
                },
                None,
            )
            .await;

        return match docs {
            Ok(cursor) => Ok(cursor
                .try_collect::<Vec<Category>>()
                .await
                .map(|categories| categories)
                .map_err(|_| "Failed to collect categories".to_string())?),
            Err(_) => Err("Failed to get categories".to_string()),
        };
    }
    pub async fn get_all(
        client: web::Data<Client>,
        user_id: ObjectId,
    ) -> Result<CategoriesResult, String> {
        let docs = client
            .database(&DB_NAME)
            .collection::<Category>(COLL_NAME)
            .find(
                doc! {
                    "userId": &user_id,
                },
                None,
            )
            .await;

        return match docs {
            Ok(cursor) => Ok(cursor
                .try_collect::<Vec<Category>>()
                .await
                .map(|categories| {
                    let income_categories = categories
                        .iter()
                        .filter(|c| matches!(c.category_type, TransactionType::Income))
                        .map(|c| c.get_details())
                        .collect();

                    let expense_categories = categories
                        .iter()
                        .filter(|c| matches!(c.category_type, TransactionType::Expense))
                        .map(|c| c.get_details())
                        .collect();

                    CategoriesResult {
                        income: income_categories,
                        expense: expense_categories,
                    }
                })
                .map_err(|_| "Failed to collect categories".to_string())?),
            Err(_) => Err("Failed to get categories".to_string()),
        };
    }

    pub async fn get(
        client: web::Data<Client>,
        _user_id: ObjectId,
        category_id: String,
    ) -> Result<CategoryDetails, String> {
        let category = match client
        .database(&DB_NAME)
        .collection::<Category>(COLL_NAME)
        .find_one(
            doc! {
                "_id": &category_id
            },
            None,
        )
        .await
    {
        Ok(category) => category,
        Err(_) => return Err("Failed to get category".to_string()),
    };

        match category {
            Some(category) => Ok(category.get_details()),
            None => Err("Category not found".to_string()),
        }
    }

    pub async fn delete(
        client: web::Data<Client>,
        _user_id: ObjectId,
        category_id: String,
    ) -> Result<(), String> {
        // TODO: delete all transactions for this category

        client
            .database(&DB_NAME)
            .collection::<Category>(COLL_NAME)
            .delete_one(
                doc! {
                    "_id": category_id,
                },
                None,
            )
            .await
            .map_or_else(|_| Err("Failed to delete category".to_string()), |_| Ok(()))
    }

    pub async fn create_default(
        client: web::Data<Client>,
        user_id: ObjectId,
    ) -> Result<(), String> {
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
            let category = Category::new(
                &CategoryData {
                    category_type: TransactionType::Income,
                    name: name.to_string(),
                    color: color.to_string(),
                    icon: name.to_string(),
                    default: true,
                },
                user_id.clone(),
            );

            create(client.clone(), category).await?;
        }

        for (name, color) in expense_categories {
            let category = Category::new(
                &CategoryData {
                    category_type: TransactionType::Expense,
                    name: name.to_string(),
                    color: color.to_string(),
                    icon: name.to_string(),
                    default: true,
                },
                user_id.clone(),
            );

            create(client.clone(), category).await?;
        }

        return Ok(());
    }
}
