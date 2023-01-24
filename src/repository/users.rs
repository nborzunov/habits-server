use actix_web::web;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson, Client};

use crate::models::errors::FormError;
use crate::models::user::{UpdateUserData, User, UserData};
use crate::services::hashing::hashing;
use crate::DB_NAME;

const COLL_NAME: &str = "users";

pub async fn create(
    client: web::Data<Client>,
    new_user: UserData,
) -> Result<User, FormError<'static>> {
    let password_hash = hashing()
        .hash_password(new_user.clone().password)
        .await
        .unwrap();

    let user_model = User::new(new_user, password_hash);

    match get_by_username(client.clone(), user_model.clone().username.unwrap()).await {
        Ok(_) => {
            return Err(FormError {
                field: "username",
                message: "User with this username already exists",
            })
        }
        Err(_) => (),
    };

    let user_id = match client
        .database(&DB_NAME)
        .collection::<User>(COLL_NAME)
        .insert_one(user_model, None)
        .await
    {
        Ok(res) => res.inserted_id.as_object_id().unwrap().clone(),
        Err(_) => {
            return Err(FormError {
                field: "",
                message: "Failed to create user",
            })
        }
    };

    Ok(get_by_id(client, user_id).await.unwrap())
}

pub async fn get_by_id(client: web::Data<Client>, id: ObjectId) -> Result<User, String> {
    let user = match client
        .database(&DB_NAME)
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
        .database(&DB_NAME)
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
        .database(&DB_NAME)
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

pub async fn change_password(
    client: web::Data<Client>,
    id: ObjectId,
    old_password: String,
    new_password: String,
) -> Result<(), FormError<'static>> {
    let user = match get_by_id(client.clone(), id).await {
        Ok(user) => user,
        Err(_) => {
            return Err(FormError {
                field: "",
                message: "Failed to change password",
            })
        }
    };

    if !hashing()
        .verify_password(&old_password, &user.password_hash)
        .await
        .unwrap()
    {
        return Err(FormError {
            field: "currentPassword",
            message: "Old password is incorrect",
        });
    }

    let new_password_hash = hashing().hash_password(new_password.clone()).await.unwrap();

    if user.password_hash == new_password_hash {
        return Err(FormError {
            field: "newPassword",
            message: "New password must be different from current",
        });
    }

    match client
        .database(&DB_NAME)
        .collection::<User>(COLL_NAME)
        .update_one(
            doc! {"_id": id },
            doc! {"$set": { "password_hash": new_password_hash } },
            None,
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(_) => {
            return Err(FormError {
                field: "",
                message: "Failed to change password",
            })
        }
    }
}
