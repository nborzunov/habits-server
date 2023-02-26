use std::pin::Pin;
use std::str::FromStr;

use crate::common::services::hashing::hashing;
use crate::users;
use crate::users::models::User;
use actix_web::error::ErrorUnauthorized;
use actix_web::web::Data;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures::future::ready;
use futures::Future;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;

#[derive(Debug)]
pub struct AuthenticationService(pub User);

impl FromRequest for AuthenticationService {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let crypto = hashing();

        let client = match req.app_data::<Data<Client>>() {
            Some(c) => c.clone(),
            None => return Box::pin(ready(Err(ErrorUnauthorized("blocked!")))),
        };

        let token = match req.headers().get("Authorization") {
            Some(auth) => auth.to_str().unwrap().replace("Bearer ", "").to_string(),
            None => return Box::pin(ready(Err(ErrorUnauthorized("Invalid token")))),
        };

        Box::pin(async move {
            match crypto.verify_jwt(token).await {
                Ok(v) => {
                    users::repository::get_by_id(client, ObjectId::from_str(&v.claims.sub).unwrap())
                        .await
                        .and_then(|u| Ok(AuthenticationService(u)))
                        .map_err(|_| ErrorUnauthorized("DB error!").into())
                }
                Err(_) => return Err(ErrorUnauthorized("blocked!")),
            }
        })
    }
}
