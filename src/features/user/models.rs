use crate::{common::models::errors::FormError, schema::users};
use actix_web::web;
use chrono::{DateTime, Utc};
use diesel::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::services::hashing::hashing;
use crate::repository::database::Database;

#[derive(Queryable, Debug, Serialize, Deserialize, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: Option<String>,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub surname: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub email_verified: bool,
    pub active: bool,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
}

impl User {
    pub async fn create(
        db: web::Data<Database>,
        user_data: NewUserData,
    ) -> Result<User, FormError<'static>> {
        let password_hash = hashing()
            .hash_password(user_data.clone().password)
            .await
            .unwrap();

        let user_form = NewUser::create(user_data, password_hash);

        let existing_user =
            Self::get_by_username(db.clone(), user_form.clone().username.unwrap()).await;

        if existing_user.is_ok() {
            return Err(FormError {
                field: "username",
                message: "profile:username.errors.alreadyExists",
            });
        }

        let new_user = diesel::insert_into(users::table)
            .values(&user_form)
            .get_result::<User>(&mut db.pool.get().unwrap());

        match new_user {
            Ok(new_user) => Ok(new_user),
            Err(_) => Err(FormError {
                field: "",
                message: "profile:errors.failedToCreate",
            }),
        }
    }

    pub async fn get_by_id(db: web::Data<Database>, id: Uuid) -> Result<User, String> {
        users::table
            .find(id)
            .first::<User>(&mut db.pool.get().unwrap())
            .map_err(|_| "User not found".to_string())
    }

    pub async fn get_by_username(
        db: web::Data<Database>,
        username: String,
    ) -> Result<User, String> {
        users::table
            .filter(users::username.eq(username))
            .first::<User>(&mut db.pool.get().unwrap())
            .map_err(|_| "User not found".to_string())
    }

    pub async fn update(
        db: web::Data<Database>,
        id: Uuid,
        user: UpdateUserData,
    ) -> Result<(), String> {
        diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(user)
            .execute(&mut db.pool.get().unwrap())
            .map(|_| ())
            .map_err(|_| "Failed to update user".to_string())
    }

    pub async fn change_password(
        db: web::Data<Database>,
        id: Uuid,
        old_password: String,
        new_password: String,
    ) -> Result<(), FormError<'static>> {
        let user = match Self::get_by_id(db.clone(), id).await {
            Ok(user) => user,
            Err(_) => {
                return Err(FormError {
                    field: "",
                    message: "profile:password.errors.failedToChange",
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
                message: "profile:password.errors.incorrect",
            });
        }

        let new_password_hash = hashing().hash_password(new_password.clone()).await.unwrap();

        if user.password_hash == new_password_hash {
            return Err(FormError {
                field: "newPassword",
                message: "profile:password.errors.sameAsOld",
            });
        }

        match diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(users::password_hash.eq(new_password_hash))
            .execute(&mut db.pool.get().unwrap())
        {
            Ok(_) => Ok(()),
            Err(_) => {
                return Err(FormError {
                    field: "",
                    message: "profile:password.errors.failedToChange",
                })
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: Option<String>,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub surname: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub email_verified: bool,
    pub active: bool,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
}

impl NewUser {
    pub fn create(new_user: NewUserData, password_hash: String) -> Self {
        Self {
            username: Some(new_user.username),
            email: new_user.email,
            password_hash,
            name: new_user.name,
            surname: new_user.surname,
            bio: None,
            image: None,
            email_verified: false,
            active: true,
            created_date: Utc::now(),
            updated_date: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewUserData {
    pub name: String,
    pub surname: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUserData {
    pub name: Option<String>,
    pub surname: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordData {
    pub current_password: String,
    pub new_password: String,
}
