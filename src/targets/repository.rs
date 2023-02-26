use actix_web::web;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;

use crate::targets::models::{Target, TargetType};
use crate::{habits, DB_NAME};

const COLL_NAME: &str = "targets";

pub async fn create(
    client: web::Data<Client>,
    user_id: ObjectId,
    target: Target,
) -> Result<(), String> {
    let habit = match habits::repository::get_by_id(client.clone(), target.habit_id.clone()).await {
        Ok(habit) => habit,
        Err(err) => return Err(err),
    };
    if habit.user_id != user_id {
        return Err("Habit does not belong to user".to_string());
    }

    let current_target_id = match target.id {
        Some(id) => client
            .database(&DB_NAME)
            .collection::<Target>(COLL_NAME)
            .find_one(doc! { "_id": id }, None)
            .await
            .unwrap()
            .map(|target| target.id)
            .unwrap(),
        None => None,
    };

    match current_target_id {
        Some(current_target_id) => match target.target_type {
            TargetType::Done => client
                .database(&DB_NAME)
                .collection::<Target>(COLL_NAME)
                .update_one(
                    doc! {
                    "habitId": target.habit_id,
                    "_id": current_target_id
                     },
                    doc! { "$set": {
                        "targetType": "done",
                        "value": target.value}
                    },
                    None,
                )
                .await
                .map(|_| ())
                .map_err(|_| "Failed to update target".to_string()),

            TargetType::Skip => client
                .database(&DB_NAME)
                .collection::<Target>(COLL_NAME)
                .update_one(
                    doc! {
                    "habitId": target.habit_id,
                    "_id": current_target_id
                     },
                    doc! { "$set": {
                        "targetType": "skip",
                        "value": 0
                    } },
                    None,
                )
                .await
                .map(|_| ())
                .map_err(|_| "Failed to update target".to_string()),

            TargetType::Empty => {
                return client
                    .database(&DB_NAME)
                    .collection::<Target>(COLL_NAME)
                    .delete_one(
                        doc! {
                            "habitId": target.habit_id,
                            "_id": current_target_id
                        },
                        None,
                    )
                    .await
                    .map(|_| ())
                    .map_err(|_| "Failed to delete target".to_string());
            }
        },
        None => match target.target_type {
            TargetType::Done => client
                .database(&DB_NAME)
                .collection(COLL_NAME)
                .insert_one(target, None)
                .await
                .map(|_| ())
                .map_err(|_| "Failed to create target".to_string()),
            TargetType::Skip => client
                .database(&DB_NAME)
                .collection(COLL_NAME)
                .insert_one(target, None)
                .await
                .map(|_| ())
                .map_err(|_| "Failed to create target".to_string()),
            TargetType::Empty => Err("Empty target".to_string()),
        },
    }
}

pub async fn get_all(
    client: web::Data<Client>,
    habit_id: &ObjectId,
) -> Result<Vec<Target>, String> {
    let docs = client
        .database(&DB_NAME)
        .collection::<Target>(COLL_NAME)
        .find(doc! { "habitId": habit_id, "deleted": false}, None)
        .await;

    return match docs {
        Ok(cursor) => cursor
            .try_collect::<Vec<Target>>()
            .await
            .map_err(|_| "Failed to collect targets".to_string()),
        Err(_) => Err("Failed to get targets".to_string()),
    };
}

pub async fn clean_data(client: web::Data<Client>, habit_id: &ObjectId) -> Result<(), String> {
    client
        .database(&DB_NAME)
        .collection::<Target>(COLL_NAME)
        .update_many(
            doc! { "habitId": habit_id},
            doc! { "$set": { "deleted": true}},
            None,
        )
        .await
        .map(|_| ())
        .map_err(|_| "Failed to clean targets".to_string())
}
