use std::cmp::Reverse;
use std::str::FromStr;

use actix_web::web;
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Bson, RawDocumentBuf};
use mongodb::{bson, Client};

use crate::models::habits::{Habit, HabitDetails};
use crate::models::targets::TargetDetails;
use crate::{repository, DB_NAME};

const COLL_NAME: &str = "habits";

pub async fn get_all(client: web::Data<Client>) -> Vec<Habit> {
    let docs: Vec<RawDocumentBuf> = client
        .database(DB_NAME)
        .collection(COLL_NAME)
        .find(None, None)
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

pub async fn delete(client: web::Data<Client>, id: String) -> Result<(), ()> {
    client
        .database(DB_NAME)
        .collection::<Habit>(COLL_NAME)
        .delete_one(
            doc! {"_id": mongodb::bson::oid::ObjectId::from_str(&id).unwrap() },
            None,
        )
        .await
        .expect("Failed to delete habit");
    Ok(())
}
