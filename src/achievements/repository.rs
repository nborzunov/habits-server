use crate::achievements::models::{
    Achievement, AchievementKey, AchievementResult, AchievementType, Progress,
};
use crate::habits::models::{Habit, HabitsAchievement};
use crate::{habits, DB_NAME};
use actix_web::web;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;
use std::cmp::Reverse;
use tokio::sync::mpsc;

const COLL_NAME: &str = "achievements";

pub async fn get_achievements(
    client: web::Data<Client>,
    origin_ref: ObjectId,
) -> Result<Vec<Achievement>, String> {
    let docs = client
        .database(&DB_NAME)
        .collection::<Achievement>(COLL_NAME)
        .find(
            doc! {
                "originRef": &origin_ref,
            },
            None,
        )
        .await;

    return match docs {
        Ok(cursor) => Ok(cursor
            .try_collect::<Vec<Achievement>>()
            .await
            .map_err(|_| "Failed to collect achievements".to_string())?),
        Err(_) => Err("Failed to get achievements".to_string()),
    };
}

pub async fn get_all(
    client: web::Data<Client>,
    user_id: ObjectId,
    habits: Vec<Habit>,
) -> Result<Vec<AchievementResult>, String> {
    let docs = client
        .database(&DB_NAME)
        .collection::<Achievement>(COLL_NAME)
        .find(
            doc! {
                "userId": &user_id,
            },
            None,
        )
        .await;

    return match docs {
        Ok(cursor) => Ok(cursor
            .try_collect::<Vec<Achievement>>()
            .await
            .map(|a| group_achievements(a, habits))
            .map_err(|_| "Failed to collect achievements".to_string())?),
        Err(_) => Err("Failed to get achievements".to_string()),
    };
}

fn group_achievements(
    achievements: Vec<Achievement>,
    habits: Vec<Habit>,
) -> Vec<AchievementResult> {
    let mut grouped_achievements = Vec::new();

    for achievement_key in HabitsAchievement::get_all() {
        let mut progress = vec![];

        let achievement_progress = achievements
            .iter()
            .copied()
            .filter(|a| a.key == AchievementKey::Habits(achievement_key))
            .collect::<Vec<Achievement>>();

        let mut completed = false;
        let mut completed_date = None;

        for achievement in achievement_progress {
            if let Some(habit) = habits.iter().find(|h| h.id == Some(achievement.origin_ref)) {
                if achievement.completed {
                    completed = true;
                    completed_date = achievement.completed_date.clone();
                }

                progress.push(Progress {
                    habit_id: habit.id.unwrap().to_string(),
                    habit_title: habit.title.clone(),
                    progress: achievement.progress,
                });
            }
        }

        progress.sort_by_key(|p| Reverse(p.progress));

        let achievement = AchievementResult {
            key: AchievementKey::Habits(achievement_key.clone()),
            achievement_type: AchievementType::Habits,
            completed_date,
            completed,
            progress,
        };
        grouped_achievements.push(achievement);
    }

    grouped_achievements
}

pub async fn create(
    client: web::Data<Client>,
    user_id: ObjectId,
    achievement_type: AchievementType,
    origin_ref: ObjectId,
) -> Result<(), String> {
    match achievement_type {
        AchievementType::Habits => {
            let mut docs = vec![];
            for achievement in HabitsAchievement::get_all() {
                docs.push(Achievement {
                    id: None,
                    key: AchievementKey::Habits(achievement.clone()),
                    achievement_type: achievement_type.clone(),
                    user_id: user_id.clone(),
                    origin_ref,
                    completed_date: None,
                    completed: false,
                    progress: 0,
                });
            }

            client
                .database(&DB_NAME)
                .collection::<Achievement>(COLL_NAME)
                .insert_many(docs, None)
                .await
                .map_or_else(
                    |_| Err("Failed to create achievements".to_string()),
                    |_| Ok(()),
                )
        }
    }
}

pub async fn check_all(
    client: web::Data<Client>,
    achievement_type: AchievementType,
    origin_ref: ObjectId,
    achievements_sender: mpsc::UnboundedSender<Vec<AchievementKey>>,
) -> Result<(), ()> {
    async move {
        match achievement_type {
            AchievementType::Habits => {
                let achievements = get_achievements(client.clone(), origin_ref)
                    .await
                    .map_err(|_| "Failed to get achievements".to_string());

                if achievements.is_err() {
                    achievements_sender.send(vec![]).unwrap();
                }

                let mut new_achievements = vec![];

                for achievement in achievements.unwrap() {
                    let habit = habits::repository::get_details(
                        client.clone(),
                        achievement.origin_ref.clone(),
                    )
                    .await
                    .unwrap();

                    let AchievementKey::Habits(key) = &achievement.key.clone();

                    let (completed, progress) = HabitsAchievement::check(key, habit);

                    if completed && !achievement.completed {
                        new_achievements.push(achievement.key.clone());
                    }
                    if progress != achievement.progress {
                        let completed_date = if completed {
                            Some(chrono::Utc::now().to_string())
                        } else {
                            None
                        };

                        client
                            .database(&DB_NAME)
                            .collection::<Achievement>(COLL_NAME)
                            .update_one(
                                doc! {
                                    "_id": achievement.id.unwrap(),
                                },
                                doc! {
                                    "$set": {
                                        "completed": completed,
                                        "completedDate": completed_date,
                                        "progress": progress
                                    }
                                },
                                None,
                            )
                            .await
                            .expect("Failed to update achievement");
                    }
                }

                achievements_sender.send(new_achievements).unwrap();
            }
        }

        return Ok(());
    }
    .await
}
