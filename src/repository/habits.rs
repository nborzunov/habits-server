use std::cmp::Reverse;

use actix_web::web;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson, Client};

use crate::models::habits::{Habit, HabitData, HabitDetails};
use crate::models::targets::TargetDetails;
use crate::{repository, DB_NAME};

const COLL_NAME: &str = "habits";

pub async fn get_all(client: web::Data<Client>, user_id: ObjectId) -> Result<Vec<Habit>, String> {
    let docs = client
        .database(&DB_NAME)
        .collection::<Habit>(COLL_NAME)
        .find(
            doc! {
                "userId": &user_id,
                "archived": false
            },
            None,
        )
        .await;

    return match docs {
        Ok(cursor) => {
            let mut habits = cursor.try_collect::<Vec<Habit>>().await.map_err(|err| {
                println!("{}", err.to_string());
                return "Failed to collect habits".to_string();
            })?;
            habits.sort_by_key(|h| Reverse(h.created_date.clone()));

            Ok(habits)
        }
        Err(_) => Err("Failed to get habits".to_string()),
    };
}

pub async fn get_by_id(client: web::Data<Client>, id: ObjectId) -> Result<Habit, String> {
    match client
        .database(&DB_NAME)
        .collection(COLL_NAME)
        .find_one(doc! { "_id": id }, None)
        .await
    {
        Ok(doc) => match doc {
            Some(doc) => {
                let habit: Habit = bson::from_document(doc).unwrap();
                Ok(habit)
            }
            None => Err("Habit not found".to_string()),
        },
        Err(_) => Err("Failed to get habit".to_string()),
    }
}

pub async fn get_details(client: web::Data<Client>, id: ObjectId) -> Result<HabitDetails, String> {
    let habit = match get_by_id(client.clone(), id).await {
        Ok(habit) => habit,
        Err(err) => return Err(err),
    };

    match repository::targets::get_all(client.clone(), &habit.id.clone().unwrap()).await {
        Ok(targets) => Ok(HabitDetails::parse(
            &habit,
            targets.iter().map(|t| TargetDetails::parse(t)).collect(),
        )),
        Err(err) => return Err(err),
    }
}

pub async fn create(client: web::Data<Client>, habit: Habit) -> Result<ObjectId, String> {
    client
        .database(&DB_NAME)
        .collection(COLL_NAME)
        .insert_one(habit, None)
        .await
        .map_or_else(
            |_| Err("Failed to create habit".to_string()),
            |result| Ok(result.inserted_id.as_object_id().unwrap().clone()),
        )
}

pub async fn edit(
    client: web::Data<Client>,
    user_id: ObjectId,
    id: ObjectId,
    habit: HabitData,
) -> Result<(), String> {
    client
        .database(&DB_NAME)
        .collection::<Habit>(COLL_NAME)
        .update_one(
            doc! {"_id": id, "userId": user_id },
            doc! {"$set": bson::to_bson(&habit).unwrap() },
            None,
        )
        .await
        .map(|_| ())
        .map_err(|_| "Failed to update habit".to_string())
}

pub async fn delete(
    client: web::Data<Client>,
    user_id: ObjectId,
    id: ObjectId,
) -> Result<(), String> {
    client
        .database(&DB_NAME)
        .collection::<Habit>(COLL_NAME)
        .delete_one(doc! {"_id": id, "userId": user_id }, None)
        .await
        .map(|_| ())
        .map_err(|_| "Failed to delete habit".to_string())
}

pub async fn archive(
    client: web::Data<Client>,
    user_id: ObjectId,
    id: ObjectId,
) -> Result<(), String> {
    client
        .database(&DB_NAME)
        .collection::<Habit>(COLL_NAME)
        .update_one(
            doc! { "_id": id, "userId": user_id },
            doc! { "$set": { "archived": true } },
            None,
        )
        .await
        .map(|_| ())
        .map_err(|_| "Failed to archive habit".to_string())
}
