use crate::features::habit::models::GridTarget;
use crate::features::habit::models::Habit;
use crate::repository::database::Database;
use crate::schema::targets;
use actix_web::web;
use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

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
#[diesel(belongs_to(Habit, foreign_key = habit_id))]
#[diesel(table_name = targets)]
pub struct Target {
    pub id: Uuid,
    pub habit_id: Uuid,
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub created_date: DateTime<Utc>,
    pub amount: i32,
    pub deleted: bool,
}

impl Target {
    pub async fn insert(
        db: web::Data<Database>,
        user_id: Uuid,
        target: TargetData,
    ) -> Result<(), String> {
        match target.id {
            Some(id) => {
                Target::update(
                    db.clone(),
                    id,
                    UpdateTargetData::create(&target, target.amount),
                )
                .await
            }
            None => Target::create(db.clone(), NewTargetData::create(&target, user_id)).await,
        }
    }

    pub async fn create(db: web::Data<Database>, target_data: NewTargetData) -> Result<(), String> {
        diesel::insert_into(targets::table)
            .values(&target_data)
            .execute(&mut db.pool.get().unwrap())
            .map(|_| ())
            .map_err(|_| "Failed to update create".to_string())
    }

    pub async fn update(
        db: web::Data<Database>,
        id: Uuid,
        target_data: UpdateTargetData,
    ) -> Result<(), String> {
        diesel::update(targets::table)
            .filter(targets::id.eq(id))
            .set(target_data)
            .execute(&mut db.pool.get().unwrap())
            .map(|_| ())
            .map_err(|_| "Failed to update target".to_string())
    }

    pub async fn delete(db: web::Data<Database>, id: Uuid) -> Result<(), String> {
        let target = targets::table.filter(targets::id.eq(id));
        diesel::delete(target)
            .execute(&mut db.pool.get().unwrap())
            .map(|_| ())
            .map_err(|_| "Error deleting target".to_string())
    }
    pub async fn get_all(db: web::Data<Database>, habit_id: &Uuid) -> Result<Vec<Target>, String> {
        targets::table
            .filter(targets::habit_id.eq(habit_id))
            .load::<Target>(&mut db.pool.get().unwrap())
            .map_err(|_| "Error loading targets".to_string())
    }

    pub async fn clean_data(db: web::Data<Database>, user_id: Uuid) -> Result<(), String> {
        diesel::update(targets::table)
            .filter(targets::user_id.eq(user_id))
            .set(targets::deleted.eq(true))
            .execute(&mut db.pool.get().unwrap())
            .map(|_| ())
            .map_err(|_| "Failed to clean targets".to_string())
    }

    pub async fn clean_habit(db: web::Data<Database>, habit_id: &Uuid) -> Result<(), String> {
        diesel::update(targets::table)
            .filter(targets::habit_id.eq(habit_id))
            .set(targets::deleted.eq(true))
            .execute(&mut db.pool.get().unwrap())
            .map(|_| ())
            .map_err(|_| "Failed to clean targets".to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable)]
#[diesel(table_name = targets)]
pub struct NewTarget {
    pub habit_id: Uuid,
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub created_date: DateTime<Utc>,
    pub amount: i32,
    pub deleted: bool,
}

impl NewTarget {
    pub fn create(data: &TargetData, user_id: Uuid) -> Self {
        Self {
            habit_id: data.habit_id.clone(),
            user_id: user_id.clone(),
            date: data.date.clone(),
            created_date: Utc::now(),
            amount: data.amount,
            deleted: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateTarget {
    pub id: Option<Uuid>,
    pub habit_id: Uuid,
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub created_date: DateTime<Utc>,
    pub amount: i32,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TargetData {
    pub id: Option<Uuid>,
    pub date: NaiveDate,
    pub habit_id: Uuid,
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = targets)]
pub struct UpdateTargetData {
    pub id: Uuid,
    pub date: NaiveDate,
    pub habit_id: Uuid,
    pub amount: i32,
}

impl UpdateTargetData {
    pub fn create(data: &TargetData, val: i32) -> Self {
        Self {
            id: data.id.clone().unwrap(),
            date: data.date.clone(),
            habit_id: data.habit_id.clone(),
            amount: val,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = targets)]
pub struct NewTargetData {
    pub date: NaiveDate,
    pub habit_id: Uuid,
    pub user_id: Uuid,
    pub amount: i32,
}

impl NewTargetData {
    pub fn create(data: &TargetData, user_id: Uuid) -> Self {
        Self {
            date: data.date.clone(),
            habit_id: data.habit_id.clone(),
            user_id: user_id.clone(),
            amount: data.amount,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TargetStatistics {
    pub current_streak_start_date: Option<DateTime<Utc>>,
    pub max_streak_count: i32,
    pub prev_streak_count: i32,
    pub current_streak_count: i32,
    pub current_streak_count_this_week: i32,
    pub current_streak_values: i32,
    pub current_streak_values_this_week: i32,
    pub failed_count: i32,
    pub failed_count_this_week: i32,
    pub skipped_count: i32,
    pub skipped_count_this_week: i32,
    pub total_count: i32,
    pub total_count_this_week: i32,
    pub total_values_count: i32,
    pub total_values_count_this_week: i32,
    pub completed_count: i32,
    pub completed_count_this_week: i32,
    pub completed_values: i32,
    pub completed_values_this_week: i32,
    pub completed_today: bool,
}

impl TargetStatistics {
    // fn new() -> Self {
    //     Self {
    //         current_streak_start_date: None,
    //         max_streak_count: 0,
    //         prev_streak_count: 0,
    //         current_streak_count: 0,
    //         current_streak_count_this_week: 0,
    //         current_streak_values: 0,
    //         current_streak_values_this_week: 0,
    //         failed_count: 0,
    //         failed_count_this_week: 0,
    //         skipped_count: 0,
    //         skipped_count_this_week: 0,
    //         total_count: 0,
    //         total_count_this_week: 0,
    //         total_values_count: 0,
    //         total_values_count_this_week: 0,
    //         completed_count: 0,
    //         completed_count_this_week: 0,
    //         completed_values: 0,
    //         completed_values_this_week: 0,
    //         completed_today: false,
    //     }
    // }

    // fn update_streak(self: &mut Self, target: &Target, days_diff: i32) {
    //     if self.current_streak_start_date.is_none() {
    //         self.current_streak_count = 1;
    //         self.current_streak_values = target.amount;
    //         self.current_streak_start_date = Some(target.date);
    //         self.current_streak_count_this_week = TargetHelper::check_prev_week(0, 1, target.date);
    //         self.current_streak_values_this_week =
    //             TargetHelper::check_prev_week(0, target.amount, target.date);
    //         self.max_streak_count = max(self.max_streak_count, self.current_streak_count);
    //     } else if days_diff == 1 {
    //         self.current_streak_count += 1;
    //         self.current_streak_values += target.amount;
    //         self.current_streak_count_this_week =
    //             TargetHelper::check_prev_week(self.current_streak_count_this_week, 1, target.date);
    //         self.current_streak_values_this_week = TargetHelper::check_prev_week(
    //             self.current_streak_values_this_week,
    //             target.amount,
    //             target.date,
    //         );
    //         self.max_streak_count = max(self.max_streak_count, self.current_streak_count);
    //     } else {
    //         self.failed_count += days_diff - 1;
    //         self.failed_count_this_week = TargetHelper::check_prev_week(
    //             self.failed_count_this_week,
    //             days_diff - 1,
    //             target.date,
    //         );
    //         self.max_streak_count = max(self.max_streak_count, self.current_streak_count);
    //         self.prev_streak_count = self.current_streak_count;
    //         self.current_streak_count = 1;
    //         self.current_streak_values = target.amount;
    //         self.current_streak_start_date = Some(target.date);
    //         self.current_streak_count_this_week = TargetHelper::check_prev_week(0, 1, target.date);
    //         self.current_streak_values_this_week =
    //             TargetHelper::check_prev_week(0, target.amount, target.date);
    //     }
    //     self.total_count += 1;
    //     self.total_count_this_week =
    //         TargetHelper::check_prev_week(self.total_count_this_week, 1, target.date);
    // }
}

pub struct TargetHelper {}
impl TargetHelper {
    // fn check_prev_week(val: i32, delta: i32, date: DateTime<Utc>) -> i32 {
    //     let today = Utc::now().date_naive();
    //     let days_diff = (today - date.date_naive()).num_days();

    //     if days_diff < 7 && days_diff >= 0 {
    //         return val + delta;
    //     }
    //     return val;
    // }

    // pub fn calculate_statistics(
    //     targets: Vec<Target>,
    //     allow_skip: bool,
    //     allow_partial_completion: bool,
    //     daily_goal: i32,
    // ) -> TargetStatistics {
    //     let today = Utc::now().date_naive();

    //     let mut statistics = TargetStatistics::new();

    //     if targets.len() == 0 {
    //         return statistics;
    //     }

    // let mut prev_date = Utc
    //     .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
    //     .unwrap()
    //     .date_naive();

    // for target in targets {
    //     let date = target.date.date_naive();
    //     let days_diff = (date - prev_date).num_days();

    //     if target.target_type == "done" {
    //         if date == today {
    //             statistics.completed_today = true;
    //         }

    //         statistics.update_streak(&target, days_diff as i32);
    //         statistics.total_values_count += target.amount;
    //         statistics.total_values_count_this_week = TargetHelper::check_prev_week(
    //             statistics.total_values_count_this_week,
    //             target.amount,
    //             target.date,
    //         );

    //         if !allow_partial_completion || target.amount >= daily_goal {
    //             statistics.completed_count += 1;
    //             statistics.completed_values += target.amount;
    //             statistics.completed_count_this_week = TargetHelper::check_prev_week(
    //                 statistics.completed_count_this_week,
    //                 1,
    //                 target.date,
    //             );
    //             statistics.completed_values_this_week = TargetHelper::check_prev_week(
    //                 statistics.completed_values_this_week,
    //                 target.amount,
    //                 target.date,
    //             );
    //         }
    //     } else if allow_skip && target.target_type == "skip" {
    //         statistics.update_streak(&target, days_diff as i32);
    //         statistics.skipped_count += 1;
    //         statistics.skipped_count_this_week = TargetHelper::check_prev_week(
    //             statistics.skipped_count_this_week,
    //             1,
    //             target.date,
    //         );
    //     } else {
    //         if statistics.current_streak_start_date.is_some() {
    //             statistics.failed_count += (date - prev_date).num_days() as i32 - 1;
    //             statistics.current_streak_count = 0;
    //             statistics.current_streak_start_date = None;
    //         }
    //     }
    //     prev_date = date;
    // }

    //     return statistics;
    // }

    pub fn get_next_mandatory_day(mandatory_days: Vec<i32>, last_date: NaiveDate) -> NaiveDate {
        let mandatory_days_set: HashSet<i32> = mandatory_days.iter().cloned().collect();
        let mut next_mandatory_day = last_date;

        loop {
            next_mandatory_day = next_mandatory_day + Duration::days(1);
            let next_mandatory_day_num = next_mandatory_day.weekday().num_days_from_sunday() as i32;

            if mandatory_days_set.contains(&next_mandatory_day_num) {
                return next_mandatory_day;
            }
        }
    }
    pub fn calculate_streaks(habit: &Habit, targets: Vec<Target>) -> (i32, i32, Vec<GridTarget>) {
        if targets.len() == 0 {
            return (0, 0, Vec::new());
        }
        let dates: Vec<NaiveDate> = targets.iter().map(|t| t.date).collect();

        let mut current_streak = 0;
        let mut longest_streak = 0;

        let mut last_date = dates.first().unwrap().clone();
        let mut grid_targets: Vec<GridTarget> = Vec::new();

        let grid_targets_map = targets
            .iter()
            .map(|t| (t.date, t))
            .collect::<HashMap<NaiveDate, &Target>>();

        if habit.frequency_type == "daily" {
            let frequency: Vec<i32> = habit
                .frequency_amount
                .as_array()
                .unwrap()
                .to_owned()
                .iter()
                .map(|v| v.as_i64().unwrap() as i32)
                .collect();

            let mut next_mandatory_day = Self::get_next_mandatory_day(frequency.clone(), last_date);
            let calc_full_week = frequency.len() == 7;

            for date in dates {
                if (calc_full_week
                    && (date == last_date + chrono::Duration::days(1) || last_date == date))
                    || (!calc_full_week && date <= next_mandatory_day)
                {
                    current_streak += 1;

                    if current_streak > longest_streak {
                        longest_streak = current_streak;
                    }
                } else {
                    if current_streak > longest_streak {
                        longest_streak = current_streak;
                    }

                    current_streak = 1;
                }

                last_date = date;
                next_mandatory_day = Self::get_next_mandatory_day(frequency.clone(), last_date);

                let target = grid_targets_map.get(&date);
                if target.is_some() {
                    let target = target.unwrap();
                    grid_targets.push(GridTarget {
                        id: target.id,
                        date: target.date,
                        amount: target.amount,
                        current_streak: current_streak,
                    });
                }
            }
        } else if habit.frequency_type == "weekly" {
            let frequency_list = habit
                .frequency_amount
                .as_array()
                .unwrap()
                .to_owned()
                .iter()
                .map(|v| v.as_i64().unwrap() as i32)
                .collect::<Vec<i32>>();

            let frequency = frequency_list.first().unwrap();
            let mut current_week = last_date.iso_week().week();
            let mut current_week_streak = 0;
            for date in dates {
                if current_week == date.iso_week().week() {
                    current_week_streak += 1;
                    current_streak += 1;

                    if current_streak > longest_streak {
                        longest_streak = current_streak;
                    }
                } else if &current_week_streak < frequency && current_week_streak != 0 {
                    current_week_streak = 0;
                    current_streak = 1;
                }

                last_date = date;
                current_week = last_date.iso_week().week();

                let target = grid_targets_map.get(&date);
                if target.is_some() {
                    let target = target.unwrap();
                    grid_targets.push(GridTarget {
                        id: target.id,
                        date: target.date,
                        amount: target.amount,
                        current_streak: current_streak,
                    });
                }
            }
        } else if habit.frequency_type == "monthly" {
            let frequency_list = habit
                .frequency_amount
                .as_array()
                .unwrap()
                .to_owned()
                .iter()
                .map(|v| v.as_i64().unwrap() as i32)
                .collect::<Vec<i32>>();

            let frequency = frequency_list.first().unwrap();
            let mut current_month = last_date.month();
            let mut current_month_streak = 0;

            for date in dates {
                if current_month == date.month() {
                    current_month_streak += 1;
                    current_streak += 1;

                    if current_streak > longest_streak {
                        longest_streak = current_streak;
                    }
                } else if &current_month_streak < frequency && current_month_streak != 0 {
                    current_month_streak = 0;
                    current_streak = 1;
                }

                last_date = date;
                current_month = last_date.month();

                let target = grid_targets_map.get(&date);
                if target.is_some() {
                    let target = target.unwrap();
                    grid_targets.push(GridTarget {
                        id: target.id,
                        date: target.date,
                        amount: target.amount,
                        current_streak: current_streak,
                    });
                }
            }
        } else if habit.frequency_type == "interval" {
            let frequency_list = habit
                .frequency_amount
                .as_array()
                .unwrap()
                .to_owned()
                .iter()
                .map(|v| v.as_i64().unwrap() as i32)
                .collect::<Vec<i32>>();

            let frequency = frequency_list.first().unwrap();

            for date in dates {
                if date - last_date <= chrono::Duration::days(*frequency as i64) {
                    current_streak += 1;

                    if current_streak > longest_streak {
                        longest_streak = current_streak;
                    }
                } else {
                    current_streak = 1;
                }

                last_date = date;

                let target = grid_targets_map.get(&date);
                if target.is_some() {
                    let target = target.unwrap();
                    grid_targets.push(GridTarget {
                        id: target.id,
                        date: target.date,
                        amount: target.amount,
                        current_streak: current_streak,
                    });
                }
            }
        }

        (current_streak, longest_streak, grid_targets)
    }
}
