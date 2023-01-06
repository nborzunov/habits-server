use actix_web::web;
use mongodb::bson::{doc, RawDocumentBuf};
use mongodb::{bson, Client};
use futures::TryStreamExt;
use std::str::FromStr;
use mongodb::bson::oid::ObjectId;


use crate::DB_NAME;
use crate::models::targets::Target;

const COLL_NAME: &str = "targets";

pub async fn create(client: web::Data<Client>, target: Target) -> Result<(), ()> {
    client.database(DB_NAME).
        collection(COLL_NAME)
        .insert_one(target, None).await.expect("Failed to insert target");
    Ok(())
}

pub async fn get_all(client: web::Data<Client>, habit_id: &ObjectId) -> Vec<Target> {
    let docs: Vec<RawDocumentBuf> = client.database(DB_NAME).
        collection(COLL_NAME)
        .find(doc! { "habitId": habit_id}, None).await
        .expect("Failed to get targets")
        .try_collect().await
        .expect("Failed to collect targets");

    docs
        .iter()
        .map(|raw| bson::from_slice(raw.as_bytes()).unwrap())
        .collect()
}