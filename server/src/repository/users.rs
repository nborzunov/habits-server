use actix_web::web;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson, Client};

use crate::models::user::{UpdateUserData, User, UserData};
use crate::services::hashing::hashing;
use crate::DB_NAME;

const COLL_NAME: &str = "users";

pub async fn create(client: web::Data<Client>, new_user: UserData) -> Result<User, String> {
    let password_hash = hashing()
        .hash_password(new_user.clone().password)
        .await
        .unwrap();

    let user_model = User::new(new_user, password_hash);

    match get_by_username(client.clone(), user_model.clone().username.unwrap()).await {
        Ok(_) => return Err("User already exists".to_string()),
        Err(_) => (),
    };
    // create validation for new user fields
    let user_id = match client
        .database(DB_NAME)
        .collection::<User>(COLL_NAME)
        .insert_one(user_model, None)
        .await
    {
        Ok(res) => res.inserted_id.as_object_id().unwrap().clone(),
        Err(_) => return Err("Failed to create user".to_string()),
    };

    get_by_id(client, user_id).await
}

pub async fn get_by_id(client: web::Data<Client>, id: ObjectId) -> Result<User, String> {
    let user = match client
        .database(DB_NAME)
        .collection::<User>(COLL_NAME)
        .find_one(
            doc! {
                "_id": &id
            },
            None,
        )
        .await
    {
        Ok(user) => user,
        Err(_) => return Err("Failed to get user".to_string()),
    };

    match user {
        Some(user) => Ok(user),
        None => Err("User not found".to_string()),
    }
}

pub async fn get_by_username(client: web::Data<Client>, username: String) -> Result<User, String> {
    let user = match client
        .database(DB_NAME)
        .collection::<User>(COLL_NAME)
        .find_one(
            doc! {
                "username": username
            },
            None,
        )
        .await
    {
        Ok(user) => user,
        Err(_) => return Err("Failed to get user".to_string()),
    };

    match user {
        Some(user) => Ok(user),
        None => Err("User not found".to_string()),
    }
}

pub async fn update(
    client: web::Data<Client>,
    id: ObjectId,
    user: UpdateUserData,
) -> Result<(), String> {
    client
        .database(DB_NAME)
        .collection::<User>(COLL_NAME)
        .update_one(
            doc! {"_id": id },
            doc! {"$set": bson::to_bson(&user).unwrap() },
            None,
        )
        .await
        .map(|_| ())
        .map_err(|_| "Failed to update user".to_string())
}
