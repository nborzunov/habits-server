use actix_web::web;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;

use crate::models::user::{User, UserData};
use crate::services::hashing::hashing;
use crate::DB_NAME;

const COLL_NAME: &str = "users";

pub async fn create(client: web::Data<Client>, new_user: UserData) -> Result<User, String> {
    let password_hash = hashing()
        .hash_password(new_user.clone().password)
        .await
        .unwrap();

    let user_model = User::new(new_user, password_hash);

    // create validation for new user fields
    match get_by_username(client.clone(), user_model.clone().username.unwrap())
        .await
        .unwrap()
    {
        Some(_) => Err("User already exists".to_string()),
        None => {
            let user_id = client
                .database(DB_NAME)
                .collection::<User>(COLL_NAME)
                .insert_one(user_model, None)
                .await
                .expect("Failed to create user")
                .inserted_id;
            Ok(get_by_id(client, user_id.as_object_id().unwrap())
                .await
                .unwrap())
        }
    }
}

pub async fn get_by_id(client: web::Data<Client>, id: ObjectId) -> Result<User, ()> {
    let user = client
        .database(DB_NAME)
        .collection::<User>(COLL_NAME)
        .find_one(
            doc! {
                "_id": &id
            },
            None,
        )
        .await
        .expect("Failed to find user")
        .expect("User not found");

    Ok(user)
}

pub async fn get_by_username(
    client: web::Data<Client>,
    username: String,
) -> mongodb::error::Result<Option<User>> {
    client
        .database(DB_NAME)
        .collection::<User>(COLL_NAME)
        .find_one(
            doc! {
                "username": username
            },
            None,
        )
        .await
}
