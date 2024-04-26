use crate::features::achievement::models::Achievement;
use crate::features::habit_target::models::{Target, TargetHelper, TargetStatistics};
use crate::features::user::models::User;
use crate::schema::{achievements, habits, habits_achievements, targets};
use actix_web::web;
use chrono::{DateTime, Utc};
use diesel::dsl::not;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

use crate::diesel::ExpressionMethods;
use crate::repository::database::Database;
use std::collections::HashMap;
use std::str::FromStr;
use tokio::sync::mpsc;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    PartialEq,
    Eq,
    Selectable,
    Identifiable,
    Associations,
)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = habits)]
pub struct Habit {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub periodicity: String, // "daily", "weekly", "monthly", "custom"
    pub periodicity_value: Option<Vec<Option<String>>>, // "sunday", "monday", "tuesday", "wednesday", "thursday", "friday", "saturday"
    pub created_date: DateTime<Utc>,
    pub goal: i32,
    pub goal_type: String, // "times", "mins"
    pub allow_skip: bool,
    pub allow_partial_completion: bool,
    pub allow_over_goal_completion: bool,
    pub can_be_finished: bool,
    pub total_goal: i32,
    pub archived: bool,
    pub deleted: bool,
}

impl Habit {
    pub async fn get_all(
        db: web::Data<Database>,
        user_id: Uuid,
    ) -> Result<Vec<HabitDetails>, String> {
        let habits_list: Vec<Habit> = habits::table
            .filter(habits::user_id.eq(user_id))
            .order(habits::created_date.desc())
            .load::<Habit>(&mut db.pool.get().unwrap())
            .unwrap();

        let targets_list: Vec<Vec<Target>> = Target::belonging_to(&habits_list)
            .order(targets::date.desc())
            .load::<Target>(&mut db.pool.get().unwrap())
            .unwrap()
            .grouped_by(&habits_list);

        let data: Vec<HabitDetails> = habits_list
            .into_iter()
            .zip(targets_list)
            .collect::<Vec<(Habit, Vec<Target>)>>()
            .into_iter()
            .map(|(h, t)| HabitDetails::parse(&h, t))
            .collect::<Vec<HabitDetails>>();

        return Ok(data);
    }

    pub async fn get_by_id(db: web::Data<Database>, id: Uuid) -> Result<Habit, String> {
        return habits::table
            .filter({
                habits::id.eq(id);
                habits::deleted.eq(false);
                habits::archived.eq(false)
            })
            .first::<Habit>(&mut db.pool.get().unwrap())
            .map_err(|_| "Habit not found".to_string());
    }

    pub async fn get_details(db: web::Data<Database>, id: Uuid) -> Result<HabitDetails, String> {
        let habit: Vec<Habit> = habits::table
            .filter(habits::id.eq(id))
            .load::<Habit>(&mut db.pool.get().unwrap())
            .unwrap();

        let targets_list: Vec<Vec<Target>> = Target::belonging_to(&habit)
            .order(targets::date.desc())
            .load::<Target>(&mut db.pool.get().unwrap())
            .unwrap()
            .grouped_by(&habit);

        let data: Vec<HabitDetails> = habit
            .into_iter()
            .zip(targets_list)
            .collect::<Vec<(Habit, Vec<Target>)>>()
            .into_iter()
            .map(|(h, t)| HabitDetails::parse(&h, t))
            .collect::<Vec<HabitDetails>>();

        return Ok(data[0].clone());
    }

    pub async fn create(db: web::Data<Database>, new_habit: NewHabit) -> Result<Uuid, String> {
        let new_habit = diesel::insert_into(habits::table)
            .values(&new_habit)
            .get_result::<Habit>(&mut db.pool.get().unwrap());

        match new_habit {
            Ok(new_habit) => {
                tokio::spawn(HabitsAchievement::create_default(
                    db.clone(),
                    new_habit.user_id.clone(),
                    new_habit.id,
                ));

                Ok(new_habit.id)
            }
            Err(_) => Err("Failed to create habit".to_string()),
        }
    }

    pub async fn edit(db: web::Data<Database>, id: Uuid, habit: HabitData) -> Result<(), String> {
        diesel::update(habits::table)
            .filter(habits::id.eq(id))
            .set(habit)
            .execute(&mut db.pool.get().unwrap())
            .map(|_| ())
            .map_err(|_| "Failed to update habit".to_string())
    }

    pub async fn delete(db: web::Data<Database>, id: Uuid) -> Result<(), String> {
        let habit = habits::table.filter(habits::id.eq(id));
        diesel::delete(habit)
            .execute(&mut db.pool.get().unwrap())
            .map(|_| ())
            .map_err(|_| "Error deleting habit".to_string())
    }

    pub async fn archive(db: web::Data<Database>, id: Uuid) -> Result<(), String> {
        diesel::update(habits::table)
            .filter(habits::id.eq(id))
            .set(habits::archived.eq(true))
            .execute(&mut db.pool.get().unwrap())
            .map(|_| ())
            .map_err(|_| "Failed to archive habit".to_string())
    }

    pub async fn delete_all_habits(db: web::Data<Database>, user_id: Uuid) -> Result<(), String> {
        diesel::update(habits::table)
            .filter(habits::user_id.eq(user_id))
            .set(habits::deleted.eq(true))
            .execute(&mut db.pool.get().unwrap())
            .map(|_| ())
            .map_err(|_| "Failed to delete habit".to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable)]
#[diesel(table_name = habits)]
pub struct NewHabit {
    pub user_id: Uuid,
    pub title: String,
    pub periodicity: String,
    pub periodicity_value: Option<Vec<Option<String>>>,
    pub created_date: DateTime<Utc>,
    pub goal: i32,
    pub goal_type: String,
    pub allow_skip: bool,
    pub allow_partial_completion: bool,
    pub allow_over_goal_completion: bool,
    pub can_be_finished: bool,
    pub total_goal: i32,
    pub archived: bool,
    pub deleted: bool,
}

impl NewHabit {
    pub fn create(new_habit: HabitData, user_id: Uuid) -> Self {
        Self {
            user_id: user_id.clone(),
            title: new_habit.title.clone(),
            periodicity: new_habit.periodicity.clone(),
            periodicity_value: new_habit.periodicity_value.clone(),
            created_date: Utc::now(),
            goal: new_habit.goal,
            goal_type: new_habit.goal_type.clone(),
            allow_skip: new_habit.allow_skip,
            allow_partial_completion: new_habit.allow_partial_completion,
            allow_over_goal_completion: new_habit.allow_over_goal_completion,
            can_be_finished: new_habit.can_be_finished,
            total_goal: new_habit.total_goal,
            archived: false,
            deleted: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HabitDetails {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    periodicity: String,
    periodicity_value: Option<Vec<Option<String>>>,
    created_date: DateTime<Utc>,
    start_date: Option<DateTime<Utc>>,
    goal: i32,
    goal_type: String,
    allow_skip: bool,
    allow_partial_completion: bool,
    allow_over_goal_completion: bool,
    can_be_finished: bool,
    total_goal: i32,
    archived: bool,
    pub statistics: TargetStatistics,
    pub targets: Vec<Target>,
}

impl HabitDetails {
    pub fn parse(h: &Habit, mut targets: Vec<Target>) -> HabitDetails {
        targets.sort_by_key(|t| t.date.clone());

        if !h.allow_skip {
            targets.retain(|t| t.target_type != "skip");
        }

        HabitDetails {
            id: h.id,
            user_id: h.user_id,
            title: h.title.clone(),
            periodicity: h.periodicity.clone(),
            periodicity_value: h.periodicity_value.clone(),
            created_date: h.created_date.clone(),
            start_date: Self::get_start_date(&targets),
            goal: h.goal.clone(),
            goal_type: h.goal_type.clone(),
            allow_skip: h.allow_skip,
            allow_partial_completion: h.allow_partial_completion,
            allow_over_goal_completion: h.allow_over_goal_completion,
            can_be_finished: h.can_be_finished,
            total_goal: h.total_goal,
            targets: targets.clone(),
            archived: h.archived,
            statistics: TargetHelper::calculate_statistics(
                targets.clone(),
                h.allow_skip,
                h.allow_partial_completion,
                h.goal,
            ),
        }
    }

    pub fn get_start_date(targets: &Vec<Target>) -> Option<DateTime<Utc>> {
        if targets.len() == 0 {
            return None;
        }
        return Some(targets[0].date.clone());
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = habits)]
pub struct HabitData {
    title: String,
    periodicity: String,
    periodicity_value: Option<Vec<Option<String>>>,
    goal: i32,
    goal_type: String,
    allow_skip: bool,
    allow_partial_completion: bool,
    allow_over_goal_completion: bool,
    can_be_finished: bool,
    total_goal: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub enum HabitsAchievementEnum {
    StreakStarter,
    HabitFormed,
    ConsistencyChampion,
    HabitualHero,
    HabitMaster,
    HabitProdigy,
    HabitLegend,
    SteadyEddie,
    Relentless,
    Unstoppable,
    SurpassingLimits,
    Perseverance,
    ComebackKid,
}
impl FromStr for HabitsAchievementEnum {
    type Err = ();

    fn from_str(input: &str) -> Result<HabitsAchievementEnum, Self::Err> {
        match input {
            "StreakStarter" => Ok(HabitsAchievementEnum::StreakStarter),
            "HabitFormed" => Ok(HabitsAchievementEnum::HabitFormed),
            "ConsistencyChampion" => Ok(HabitsAchievementEnum::ConsistencyChampion),
            "HabitualHero" => Ok(HabitsAchievementEnum::HabitualHero),
            "HabitMaster" => Ok(HabitsAchievementEnum::HabitMaster),
            "HabitProdigy" => Ok(HabitsAchievementEnum::HabitProdigy),
            "HabitLegend" => Ok(HabitsAchievementEnum::HabitLegend),
            "SteadyEddie" => Ok(HabitsAchievementEnum::SteadyEddie),
            "Relentless" => Ok(HabitsAchievementEnum::Relentless),
            "Unstoppable" => Ok(HabitsAchievementEnum::Unstoppable),
            "SurpassingLimits" => Ok(HabitsAchievementEnum::SurpassingLimits),
            "Perseverance" => Ok(HabitsAchievementEnum::Perseverance),
            "ComebackKid" => Ok(HabitsAchievementEnum::ComebackKid),
            _ => Err(()),
        }
    }
}

impl fmt::Display for HabitsAchievementEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl HabitsAchievementEnum {
    pub fn goal(&self) -> Option<i32> {
        match self {
            Self::StreakStarter => Some(3),
            Self::HabitFormed => Some(7),
            Self::ConsistencyChampion => Some(14),
            Self::HabitualHero => Some(30),
            Self::HabitMaster => Some(60),
            Self::HabitProdigy => Some(90),
            Self::HabitLegend => Some(180),
            Self::SteadyEddie => Some(21),
            Self::Relentless => Some(30),
            Self::Unstoppable => Some(60),
            _ => None,
        }
    }

    pub fn get_all() -> Vec<Self> {
        vec![
            Self::StreakStarter,
            Self::HabitFormed,
            Self::ConsistencyChampion,
            Self::HabitualHero,
            Self::HabitMaster,
            Self::HabitProdigy,
            Self::HabitLegend,
            Self::SteadyEddie,
            Self::Relentless,
            Self::Unstoppable,
            Self::SurpassingLimits,
            Self::Perseverance,
            Self::ComebackKid,
        ]
    }

    pub fn check(key: &HabitsAchievementEnum, habit: HabitDetails) -> (bool, i32) {
        let mut completed = false;

        return match key {
            HabitsAchievementEnum::StreakStarter
            | HabitsAchievementEnum::HabitFormed
            | HabitsAchievementEnum::ConsistencyChampion
            | HabitsAchievementEnum::HabitualHero
            | HabitsAchievementEnum::HabitMaster
            | HabitsAchievementEnum::HabitProdigy
            | HabitsAchievementEnum::HabitLegend => {
                let goal = key.goal();
                if goal.is_some() && habit.statistics.max_streak_count >= goal.unwrap() {
                    completed = true;
                }
                (completed, habit.statistics.max_streak_count)
            }
            HabitsAchievementEnum::SteadyEddie
            | HabitsAchievementEnum::Relentless
            | HabitsAchievementEnum::Unstoppable => {
                let goal = key.goal();
                if goal.is_some()
                    && habit.statistics.max_streak_count >= goal.unwrap()
                    && habit.statistics.failed_count == 0
                {
                    completed = true;
                }

                (completed, habit.statistics.completed_count)
            }
            HabitsAchievementEnum::SurpassingLimits => {
                if habit.statistics.current_streak_count >= habit.statistics.prev_streak_count
                    && habit.statistics.prev_streak_count > 0
                {
                    completed = true;
                }

                (completed, habit.statistics.current_streak_count)
            }
            HabitsAchievementEnum::Perseverance | HabitsAchievementEnum::ComebackKid => {
                let goal = key.goal();
                if goal.is_some()
                    && habit.statistics.current_streak_count >= goal.unwrap()
                    && habit.statistics.failed_count > 0
                {
                    completed = true;
                }

                (completed, habit.statistics.current_streak_count)
            }
        };
    }
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Selectable,
    Identifiable,
    Associations,
    Queryable,
    PartialEq,
)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Achievement, foreign_key = achievement_id))]
#[diesel(table_name = habits_achievements)]
pub struct HabitsAchievement {
    pub id: Uuid,
    pub user_id: Uuid,
    pub achievement_id: Uuid,
    pub habit_id: Uuid,
    pub progress: i32,
}

impl HabitsAchievement {
    pub async fn create_default(
        db: web::Data<Database>,
        user_id: Uuid,
        habit_id: Uuid,
    ) -> Result<(), String> {
        let achievements_map = Achievement::get_achievements(db.clone(), user_id)
            .await
            .unwrap()
            .into_iter()
            .map(|item| (item.clone().key, item))
            .collect::<HashMap<String, Achievement>>();

        let achievements_to_insert = HabitsAchievementEnum::get_all()
            .iter()
            .map(|a| {
                let key = &a.to_string();

                NewHabitsAchievement::create(
                    HabitsAchievementData {
                        achievement_id: achievements_map.get(key).unwrap().id,
                        habit_id: habit_id.clone(),
                        progress: 0,
                    },
                    user_id.clone(),
                )
            })
            .collect::<Vec<NewHabitsAchievement>>();

        diesel::insert_into(habits_achievements::table)
            .values(achievements_to_insert)
            .execute(&mut db.pool.get().unwrap())
            .unwrap();

        Ok(())
    }

    pub async fn get_all(
        db: web::Data<Database>,
        habit_id: Uuid,
    ) -> Result<Vec<(HabitsAchievement, Achievement)>, String> {
        habits_achievements::table
            .filter(habits_achievements::habit_id.eq(habit_id))
            .inner_join(
                achievements::table.on(achievements::id.eq(habits_achievements::achievement_id)),
            )
            .select((HabitsAchievement::as_select(), Achievement::as_select()))
            .load::<(HabitsAchievement, Achievement)>(&mut db.pool.get().unwrap())
            .map_err(|_| "Error loading achievements".to_string())
    }
    pub async fn check_all(
        db: web::Data<Database>,
        achievements_sender: mpsc::UnboundedSender<Vec<String>>,
        user_id: Uuid,
        habit_id: Uuid,
    ) -> Result<(), ()> {
        async move {
            let achievements = HabitsAchievement::get_all(db.clone(), habit_id)
                .await
                .map_err(|_| "Failed to get achievements".to_string());

            if achievements.is_err() {
                achievements_sender.send(vec![]).unwrap();
            }

            let mut new_achievements = vec![];

            let habits_map = Habit::get_all(db.clone(), user_id.clone())
                .await
                .map_err(|_| "Failed to get habits".to_string())
                .unwrap()
                .into_iter()
                .map(|item| (item.id, item))
                .collect::<HashMap<Uuid, HabitDetails>>();

            let habit = Habit::get_details(db.clone(), habit_id.clone())
                .await
                .map_err(|_| "Failed to get habit".to_string())
                .unwrap();

            for (habit_achievement, achievement) in achievements.unwrap() {
                let (completed, progress) = HabitsAchievementEnum::check(
                    &HabitsAchievementEnum::from_str(&achievement.key).unwrap(),
                    habit.clone(),
                );

                if progress != habit_achievement.progress {
                    diesel::update(habits_achievements::table)
                        .filter(habits_achievements::id.eq(habit_achievement.id))
                        .set(habits_achievements::progress.eq(progress))
                        .execute(&mut db.pool.get().unwrap())
                        .map(|_| ())
                        .map_err(|_| "Failed to update achievement".to_string())
                        .unwrap();
                }

                if completed == achievement.completed {
                    continue;
                };

                let completed_date = if completed {
                    Some(chrono::Utc::now())
                } else {
                    None
                };

                let other_habits_achievements: Vec<(bool, i32)> = habits_achievements::table
                    .filter(habits_achievements::achievement_id.eq(achievement.id))
                    .filter(not(habits_achievements::id.eq(habit_achievement.id)))
                    .load::<HabitsAchievement>(&mut db.pool.get().unwrap())
                    .unwrap()
                    .into_iter()
                    .map(|other_habit| {
                        HabitsAchievementEnum::check(
                            &HabitsAchievementEnum::from_str(&achievement.key).unwrap(),
                            habits_map.get(&other_habit.habit_id).unwrap().clone(),
                        )
                    })
                    .collect();

                let need_to_notify = completed
                    && other_habits_achievements
                        .clone()
                        .into_iter()
                        .filter(|(other_habit_completed, _)| *other_habit_completed)
                        .count()
                        == 0;

                if need_to_notify {
                    new_achievements.push(achievement.key.clone())
                }

                let need_to_update = (completed && !achievement.completed)
                    || (!completed
                        && other_habits_achievements
                            .clone()
                            .into_iter()
                            .filter(|(other_habit_completed, _)| *other_habit_completed)
                            .count()
                            == 0);

                if need_to_update {
                    diesel::update(achievements::table)
                        .filter(achievements::id.eq(habit_achievement.achievement_id))
                        .set((
                            achievements::completed.eq(completed),
                            achievements::completed_date.eq(completed_date),
                        ))
                        .execute(&mut db.pool.get().unwrap())
                        .map(|_| ())
                        .map_err(|_| "Failed to update achievement".to_string())
                        .unwrap();
                }
            }

            achievements_sender.send(new_achievements).unwrap();
        }
        .await;

        return Ok(());
    }
}

pub struct HabitsAchievementDetails {
    pub id: Uuid,
    pub user_id: Uuid,
    pub achievement: Achievement,
    pub habit: Habit,
    pub progress: i32,
}

impl From<(HabitsAchievement, Achievement, Habit)> for HabitsAchievementDetails {
    fn from(t: (HabitsAchievement, Achievement, Habit)) -> Self {
        Self {
            id: t.0.id,
            user_id: t.0.user_id,
            achievement: t.1,
            habit: t.2,
            progress: t.0.progress,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable)]
#[diesel(table_name = habits_achievements)]
pub struct NewHabitsAchievement {
    pub user_id: Uuid,
    pub achievement_id: Uuid,
    pub habit_id: Uuid,
    pub progress: i32,
}

impl NewHabitsAchievement {
    pub fn create(new_habit_achievement: HabitsAchievementData, user_id: Uuid) -> Self {
        Self {
            user_id: user_id.clone(),
            achievement_id: new_habit_achievement.achievement_id.clone(),
            habit_id: new_habit_achievement.habit_id.clone(),
            progress: new_habit_achievement.progress,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = habits_achievements)]
pub struct HabitsAchievementData {
    achievement_id: Uuid,
    habit_id: Uuid,
    progress: i32,
}
