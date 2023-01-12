use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: Option<String>,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub email_verified: bool,
    pub active: bool,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
}

impl User {
    pub fn new(new_user: UserData, password_hash: String) -> Self {
        Self {
            id: None,
            username: Some(new_user.username),
            email: new_user.email,
            password_hash,
            full_name: None,
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
#[serde(rename_all = "camelCase")]
pub struct UserDetails {
    pub id: String,
    pub username: Option<String>,
    pub email: String,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub email_verified: bool,
    pub active: bool,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
}

impl UserDetails {
    pub fn parse(u: &User) -> UserDetails {
        UserDetails {
            id: u.id.clone().unwrap().to_string(),
            username: u.username.clone(),
            email: u.email.clone(),
            full_name: u.full_name.clone(),
            bio: u.bio.clone(),
            image: u.image.clone(),
            email_verified: u.email_verified,
            active: u.active,
            created_date: u.created_date,
            updated_date: u.updated_date,
        }
    }
}

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct UserData {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3))]
    pub password: String,
}
