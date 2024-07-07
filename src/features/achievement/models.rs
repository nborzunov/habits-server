use crate::diesel::ExpressionMethods;
use crate::features::habit::models::{HabitDetails, HabitsAchievement, HabitsAchievementEnum};
use crate::features::user::models::User;
use crate::schema::{achievements, habits_achievements};
use crate::{features::habit::models::Habit, repository::database::Database};
use actix_web::web;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::cmp::Reverse;
use std::collections::HashMap;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Identifiable,
    Associations,
    PartialEq,
    Eq,
)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = achievements)]
pub struct Achievement {
    pub id: Uuid,
    pub user_id: Uuid,
    pub a_order: i32,
    pub key: String,
    pub achievement_type: String, // "habits",
    pub completed_date: Option<DateTime<Utc>>,
    pub completed: bool,
    pub created_date: DateTime<Utc>,
}

impl Achievement {
    pub async fn get_achievements(
        db: web::Data<Database>,
        user_id: Uuid,
    ) -> Result<Vec<Achievement>, String> {
        achievements::table
            .filter(achievements::user_id.eq(user_id))
            .load::<Achievement>(&mut db.pool.get().unwrap())
            .map_err(|_| "Error loading achievements".to_string())
    }

    pub async fn get_all(
        db: web::Data<Database>,
        user_id: Uuid,
    ) -> Result<Vec<AchievementResult>, String> {
        let achievements: Vec<(Achievement, Option<HabitsAchievement>)> = achievements::table
            .filter(achievements::user_id.eq(user_id))
            .order(achievements::a_order.desc())
            .left_join(
                habits_achievements::table
                    .on(habits_achievements::achievement_id.eq(achievements::id)),
            )
            .load::<(Achievement, Option<HabitsAchievement>)>(&mut db.pool.get().unwrap())
            .unwrap();

        let achievements_map: HashMap<Uuid, Achievement> = achievements
            .clone()
            .into_iter()
            .map(|(a, _)| (a.id, a))
            .collect();
        let habits_achievements_map: HashMap<Uuid, HabitsAchievement> = achievements
            .clone()
            .into_iter()
            .filter(|(_, a)| a.is_some())
            .map(|(_, a)| (a.clone().unwrap().id, a.unwrap()))
            .collect();

        let habits_map: HashMap<Uuid, HabitDetails> = Habit::get_all(db.clone(), user_id.clone())
            .await?
            .clone()
            .into_iter()
            .map(|h| (h.id, h))
            .collect();

        let mut grouped_achievements: Vec<AchievementResult> = vec![];

        for (_, achievement) in achievements_map.into_iter() {
            let mut progress = vec![];

            let mut completed = false;
            let mut completed_date = None;

            for (_, habits_achievement) in habits_achievements_map
                .clone()
                .into_iter()
                .filter(|(_, a)| a.achievement_id == achievement.id)
            {
                if let Some(habit) = habits_map.get(&habits_achievement.habit_id) {
                    if achievement.completed {
                        completed = true;
                        completed_date = achievement.completed_date.clone();
                    }

                    progress.push(Progress {
                        habit_id: habit.id,
                        habit_name: habit.name.clone(),
                        progress: habits_achievement.progress,
                    });
                }
            }

            progress.sort_by_key(|p| Reverse(p.progress));

            let achievement = AchievementResult {
                a_order: achievement.a_order,
                key: achievement.key.clone(),
                achievement_type: "habits".to_string(),
                completed_date,
                completed,
                progress,
                created_date: achievement.created_date,
            };
            grouped_achievements.push(achievement);
        }

        grouped_achievements.sort_by_key(|a| a.a_order);

        Ok(grouped_achievements)
    }

    pub async fn create_default(
        db: web::Data<Database>,
        user_id: Uuid,
        achievement_type: String,
    ) -> Result<(), String> {
        match achievement_type.as_str() {
            "habits" => {
                let achievements_to_insert = HabitsAchievementEnum::get_all()
                    .iter()
                    .enumerate()
                    .map(|(index, a)| {
                        let key = &a.to_string();

                        NewAchivement {
                            user_id: user_id.clone(),
                            a_order: index as i32,
                            key: key.clone(),
                            achievement_type: achievement_type.clone(),
                            completed_date: None,
                            completed: false,
                        }
                    })
                    .collect::<Vec<NewAchivement>>();

                diesel::insert_into(achievements::table)
                    .values(achievements_to_insert)
                    .execute(&mut db.pool.get().unwrap())
                    .unwrap();

                Ok(())
            }
            _ => Err("Achievement type not found".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable)]
#[diesel(table_name = achievements)]
pub struct NewAchivement {
    pub a_order: i32,
    pub user_id: Uuid,
    pub key: String,
    pub achievement_type: String, // "habits",
    pub completed_date: Option<DateTime<Utc>>,
    pub completed: bool,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AchievementResult {
    pub a_order: i32,
    pub key: String,
    pub achievement_type: String,
    pub completed: bool,
    pub completed_date: Option<DateTime<Utc>>,
    pub progress: Vec<Progress>,
    pub created_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Progress {
    pub habit_id: Uuid,
    pub habit_name: String,
    pub progress: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = achievements)]
struct UpdateAchievement {
    pub completed: bool,
    pub completed_date: Option<DateTime<Utc>>,
}
