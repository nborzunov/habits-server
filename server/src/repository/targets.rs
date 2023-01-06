use actix_web::web;
use futures::TryStreamExt;
use mongodb::{bson, Client};
use mongodb::bson::{doc, RawDocumentBuf};
use mongodb::bson::oid::ObjectId;

use crate::DB_NAME;
use crate::models::targets::{Target, TargetType};

const COLL_NAME: &str = "targets";

pub async fn create(client: web::Data<Client>, target: Target) -> Result<(), ()> {
    match target.target_type {
        TargetType::Done => {
            client.database(DB_NAME)
                .collection(COLL_NAME)
                .insert_one(target, None)
                .await.expect("Failed to insert target");
        }
        TargetType::Skip => {
            client.database(DB_NAME)
                .collection::<Target>(COLL_NAME)
                .update_one(
                    doc! { "_id": target.id.unwrap() },
                    doc! { "$set": { "targetType": "skip" } },
                    None,
                )
                .await.expect("Failed to update target");
        }
        TargetType::Empty => {
            client.database(DB_NAME)
                .collection::<Target>(COLL_NAME)
                .delete_one(doc! { "_id": target.id.unwrap() }, None)
                .await.expect("Failed to delete target");
        }
    }

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