use std::sync::Arc;

use actix_web::web::block;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use pwhash::bcrypt;
use pwhash::bcrypt::BcryptSetup;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CryptoService {
    pub key: Arc<String>,
    pub jwt_secret: Arc<String>,
}

impl CryptoService {
    pub async fn hash_password(&self, password: String) -> Result<String, ()> {
        match bcrypt::hash_with(
            BcryptSetup {
                salt: Some(&self.key.clone()),
                ..Default::default()
            },
            password,
        ) {
            Ok(res) => Ok(res),
            _ => Err(()),
        }
    }

    pub async fn verify_password(&self, password: &str, password_hash: &str) -> Result<bool, ()> {
        Ok(bcrypt::verify(password, password_hash))
    }

    pub async fn generate_jwt(&self, user_id: Uuid) -> Result<String, String> {
        let jwt_key = self.jwt_secret.clone();
        block(move || {
            let headers = Header::default();
            let encoding_key = EncodingKey::from_secret(jwt_key.as_bytes());
            let now = Utc::now() + Duration::days(1); // Expires in 1 day
            let claims = Claims {
                sub: user_id.to_string(),
                exp: now.timestamp(),
            };
            encode(&headers, &claims, &encoding_key).unwrap()
        })
        .await
        .map_err(|_err| "error generating jwt".to_string())
    }

    pub async fn verify_jwt(&self, token: String) -> Result<TokenData<Claims>, String> {
        let jwt_key = self.jwt_secret.clone();
        block(move || {
            let decoding_key = DecodingKey::from_secret(jwt_key.as_bytes());
            let validation = Validation::default();
            decode::<Claims>(&token, &decoding_key, &validation).unwrap()
        })
        .await
        .map_err(|_err| "error verifying jwt".to_string())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    // aud
    // role
    // perms
}

#[derive(Serialize)]
pub struct Auth {
    pub token: String,
}
