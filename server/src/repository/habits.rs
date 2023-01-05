use actix_web::web;
use mongodb::bson::{doc, RawDocumentBuf};
use mongodb::{bson, Client};
use futures::TryStreamExt;
use std::str::FromStr;

use crate::models::habits::{Habit};

const DB_NAME: &str = "dev";
const COLL_NAME: &str = "habits";

pub async fn get_all(client: web::Data<Client>) -> Vec<Habit> {
    let docs: Vec<RawDocumentBuf> = client.database(DB_NAME).
        collection(COLL_NAME)
        .find(None, None).await
        .expect("Failed to get docs")
        .try_collect().await
        .expect("Failed to collect docs");

    docs
        .iter()
        .map(|raw| bson::from_slice(raw.as_bytes()).unwrap())
        .collect()
}

pub async fn create(client: web::Data<Client>, habit: Habit) -> Result<(), ()> {
    client.database(DB_NAME).
        collection(COLL_NAME)
        .insert_one(habit, None).await.expect("Failed to insert doc");
    Ok(())
}

pub async fn delete(client: web::Data<Client>, id: String) -> Result<(), ()> {
    client.database(DB_NAME).collection::<Habit>(COLL_NAME)
        .delete_one(doc! {"_id": mongodb::bson::oid::ObjectId::from_str(&id).unwrap()}, None)
        .await.expect("Failed to delete doc");
    Ok(())
}