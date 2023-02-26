use crate::common::services::crypto::CryptoService;
use std::env;
use std::sync::Arc;

pub fn hashing() -> CryptoService {
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    CryptoService {
        key: Arc::new(secret_key.clone()),
        jwt_secret: Arc::new(jwt_secret.clone()),
    }
}
