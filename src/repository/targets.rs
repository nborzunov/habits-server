use actix_web::web;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;

use crate::models::targets::{Target, TargetType};
use crate::{repository, DB_NAME};

const COLL_NAME: &str = "targets";

pub async fn create(
    client: web::Data<Client>,
    user_id: ObjectId,
    target: Target,
) -> Result<(), String> {
    let habit = match repository::habits::get_by_id(client.clone(), target.habit_id.clone()).await {
        Ok(habit) => habit,
        Err(err) => return Err(err),
    };
    if habit.user_id != user_id {
        return Err("Habit does not belong to user".to_string());
    }
    let result = match target.target_type {
        TargetType::Done => client
            .database(DB_NAME)
            .collection(COLL_NAME)
            .insert_one(target, None)
            .await
            .map(|_| ())
            .map_err(|_| "Failed to create target".to_string()),
        TargetType::Skip => client
            .database(DB_NAME)
            .collection::<Target>(COLL_NAME)
            .update_one(
                doc! { "_id": target.id.unwrap() },
                doc! { "$set": { "targetType": "skip" } },
                None,
            )
            .await
            .map(|_| ())
            .map_err(|_| "Failed to update target".to_string()),
        TargetType::Empty => client
            .database(DB_NAME)
            .collection::<Target>(COLL_NAME)
            .delete_one(doc! { "_id": target.id.unwrap() }, None)
            .await
            .map(|_| ())
            .map_err(|_| "Failed to delete target".to_string()),
    };

    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

pub async fn get_all(
    client: web::Data<Client>,
    habit_id: &ObjectId,
) -> Result<Vec<Target>, String> {
    let docs = client
        .database(DB_NAME)
        .collection::<Target>(COLL_NAME)
        .find(doc! { "habitId": habit_id}, None)
        .await;

    return match docs {
        Ok(cursor) => cursor
            .try_collect::<Vec<Target>>()
            .await
            .map_err(|_| "Failed to collect targets".to_string()),
        Err(_) => Err("Failed to get targets".to_string()),
    };
}
