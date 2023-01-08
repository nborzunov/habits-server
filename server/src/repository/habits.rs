use std::cmp::Reverse;
use std::str::FromStr;

use actix_web::web;
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Bson, RawDocumentBuf};
use mongodb::results::UpdateResult;
use mongodb::{bson, Client};

use crate::models::habits::{Habit, HabitData, HabitDetails};
use crate::models::targets::TargetDetails;
use crate::{repository, DB_NAME};

const COLL_NAME: &str = "habits";

pub async fn get_all(client: web::Data<Client>) -> Vec<Habit> {
    let docs: Vec<RawDocumentBuf> = client
        .database(DB_NAME)
        .collection(COLL_NAME)
        .find(
            doc! {
                "archived": false
            },
            None,
        )
        .await
        .expect("Failed to get habits")
        .try_collect()
        .await
        .expect("Failed to collect habits");

    let mut habits: Vec<Habit> = docs
        .iter()
        .map(|raw| bson::from_slice(raw.as_bytes()).unwrap())
        .collect();

    habits.sort_by_key(|h| Reverse(h.created_date.clone()));

    habits
}

pub async fn get_by_id(client: web::Data<Client>, id: ObjectId) -> Habit {
    client
        .database(DB_NAME)
        .collection(COLL_NAME)
        .find_one(doc! { "_id": id }, None)
        .await
        .expect("Failed to get habit")
        .unwrap()
}

pub async fn get_details(client: web::Data<Client>, id: ObjectId) -> HabitDetails {
    let habit = get_by_id(client.clone(), id).await;
    let targets = repository::targets::get_all(client.clone(), &habit.id.clone().unwrap()).await;

    HabitDetails::parse(
        &habit,
        targets.iter().map(|t| TargetDetails::parse(&t)).collect(),
    )
}

pub async fn create(client: web::Data<Client>, habit: Habit) -> Result<Bson, ()> {
    Ok(client
        .database(DB_NAME)
        .collection(COLL_NAME)
        .insert_one(habit, None)
        .await
        .expect("Failed to insert habit")
        .inserted_id)
}

pub async fn edit(client: web::Data<Client>, id: String, habit: HabitData) -> UpdateResult {
    client
        .database(DB_NAME)
        .collection::<Habit>(COLL_NAME)
        .update_one(
            doc! {"_id": ObjectId::from_str(&id).unwrap() },
            doc! {"$set": bson::to_bson(&habit).unwrap() },
            None,
        )
        .await
        .expect("Failed to edit habit")
}

pub async fn delete(client: web::Data<Client>, id: String) -> Result<(), ()> {
    client
        .database(DB_NAME)
        .collection::<Habit>(COLL_NAME)
        .delete_one(doc! {"_id": ObjectId::from_str(&id).unwrap() }, None)
        .await
        .expect("Failed to delete habit");
    Ok(())
}

pub async fn archive(client: web::Data<Client>, id: String) -> Result<(), ()> {
    client
        .database(DB_NAME)
        .collection::<Habit>(COLL_NAME)
        .update_one(
            doc! { "_id": ObjectId::from_str(&id).unwrap() },
            doc! { "$set": { "archived": true } },
            None,
        )
        .await
        .expect("Failed to archive habit");
    Ok(())
}
