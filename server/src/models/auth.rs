use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}
