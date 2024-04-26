use crate::features::habit::models::Habit;
use crate::repository::database::Database;
use crate::schema::targets;
use actix_web::web;
use chrono::{DateTime, TimeZone, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::max;
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
    pub date: DateTime<Utc>,
    pub created_date: DateTime<Utc>,
    pub target_type: String, // "done", "skip", "empty"
    pub value: i32,
    pub deleted: bool,
}

impl Target {
    pub async fn insert(
        db: web::Data<Database>,
        user_id: Uuid,
        target: TargetData,
    ) -> Result<(), String> {
        match target.id {
            Some(target_id) => match target.target_type.as_str() {
                "done" => {
                    Target::update(
                        db.clone(),
                        target_id,
                        UpdateTargetData::create(&target, target.value),
                    )
                    .await
                }

                "skip" => {
                    Target::update(
                        db.clone(),
                        target_id,
                        UpdateTargetData::create(&target, target.value),
                    )
                    .await
                }

                "empty" => Target::delete(db.clone(), target_id).await,
                _ => Err("Invalid target type".to_string()),
            },
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
    pub date: DateTime<Utc>,
    pub created_date: DateTime<Utc>,
    pub target_type: String,
    pub value: i32,
    pub deleted: bool,
}

impl NewTarget {
    pub fn create(data: &TargetData, user_id: Uuid) -> Self {
        Self {
            habit_id: data.habit_id.clone(),
            user_id: user_id.clone(),
            date: data.date.clone(),
            created_date: Utc::now(),
            target_type: data.target_type.clone(),
            value: data.value,
            deleted: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateTarget {
    pub id: Option<Uuid>,
    pub habit_id: Uuid,
    pub user_id: Uuid,
    pub date: DateTime<Utc>,
    pub created_date: DateTime<Utc>,
    pub target_type: String,
    pub value: i32,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TargetData {
    pub id: Option<Uuid>,
    pub date: DateTime<Utc>,
    pub target_type: String,
    pub habit_id: Uuid,
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = targets)]
pub struct UpdateTargetData {
    pub id: Uuid,
    pub date: DateTime<Utc>,
    pub target_type: String,
    pub habit_id: Uuid,
    pub value: i32,
}

impl UpdateTargetData {
    pub fn create(data: &TargetData, val: i32) -> Self {
        Self {
            id: data.id.clone().unwrap(),
            date: data.date.clone(),
            target_type: data.target_type.clone(),
            habit_id: data.habit_id.clone(),
            value: val,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[diesel(table_name = targets)]
pub struct NewTargetData {
    pub date: DateTime<Utc>,
    pub target_type: String,
    pub habit_id: Uuid,
    pub user_id: Uuid,
    pub value: i32,
}

impl NewTargetData {
    pub fn create(data: &TargetData, user_id: Uuid) -> Self {
        Self {
            date: data.date.clone(),
            target_type: data.target_type.clone(),
            habit_id: data.habit_id.clone(),
            user_id: user_id.clone(),
            value: data.value,
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
    fn new() -> Self {
        Self {
            current_streak_start_date: None,
            max_streak_count: 0,
            prev_streak_count: 0,
            current_streak_count: 0,
            current_streak_count_this_week: 0,
            current_streak_values: 0,
            current_streak_values_this_week: 0,
            failed_count: 0,
            failed_count_this_week: 0,
            skipped_count: 0,
            skipped_count_this_week: 0,
            total_count: 0,
            total_count_this_week: 0,
            total_values_count: 0,
            total_values_count_this_week: 0,
            completed_count: 0,
            completed_count_this_week: 0,
            completed_values: 0,
            completed_values_this_week: 0,
            completed_today: false,
        }
    }

    fn update_streak(self: &mut Self, target: &Target, days_diff: i32) {
        if self.current_streak_start_date.is_none() {
            self.current_streak_count = 1;
            self.current_streak_values = target.value;
            self.current_streak_start_date = Some(target.date);
            self.current_streak_count_this_week = TargetHelper::check_prev_week(0, 1, target.date);
            self.current_streak_values_this_week =
                TargetHelper::check_prev_week(0, target.value, target.date);
            self.max_streak_count = max(self.max_streak_count, self.current_streak_count);
        } else if days_diff == 1 {
            self.current_streak_count += 1;
            self.current_streak_values += target.value;
            self.current_streak_count_this_week =
                TargetHelper::check_prev_week(self.current_streak_count_this_week, 1, target.date);
            self.current_streak_values_this_week = TargetHelper::check_prev_week(
                self.current_streak_values_this_week,
                target.value,
                target.date,
            );
            self.max_streak_count = max(self.max_streak_count, self.current_streak_count);
        } else {
            self.failed_count += days_diff - 1;
            self.failed_count_this_week = TargetHelper::check_prev_week(
                self.failed_count_this_week,
                days_diff - 1,
                target.date,
            );
            self.max_streak_count = max(self.max_streak_count, self.current_streak_count);
            self.prev_streak_count = self.current_streak_count;
            self.current_streak_count = 1;
            self.current_streak_values = target.value;
            self.current_streak_start_date = Some(target.date);
            self.current_streak_count_this_week = TargetHelper::check_prev_week(0, 1, target.date);
            self.current_streak_values_this_week =
                TargetHelper::check_prev_week(0, target.value, target.date);
        }
        self.total_count += 1;
        self.total_count_this_week =
            TargetHelper::check_prev_week(self.total_count_this_week, 1, target.date);
    }
}

pub struct TargetHelper {}
impl TargetHelper {
    fn check_prev_week(val: i32, delta: i32, date: DateTime<Utc>) -> i32 {
        let today = Utc::now().date_naive();
        let days_diff = (today - date.date_naive()).num_days();

        if days_diff < 7 && days_diff >= 0 {
            return val + delta;
        }
        return val;
    }

    pub fn calculate_statistics(
        targets: Vec<Target>,
        allow_skip: bool,
        allow_partial_completion: bool,
        daily_goal: i32,
    ) -> TargetStatistics {
        let today = Utc::now().date_naive();

        let mut statistics = TargetStatistics::new();

        if targets.len() == 0 {
            return statistics;
        }

        let mut prev_date = Utc
            .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
            .unwrap()
            .date_naive();

        for target in targets {
            let date = target.date.date_naive();
            let days_diff = (date - prev_date).num_days();

            if target.target_type == "done" {
                if date == today {
                    statistics.completed_today = true;
                }

                statistics.update_streak(&target, days_diff as i32);
                statistics.total_values_count += target.value;
                statistics.total_values_count_this_week = TargetHelper::check_prev_week(
                    statistics.total_values_count_this_week,
                    target.value,
                    target.date,
                );

                if !allow_partial_completion || target.value >= daily_goal {
                    statistics.completed_count += 1;
                    statistics.completed_values += target.value;
                    statistics.completed_count_this_week = TargetHelper::check_prev_week(
                        statistics.completed_count_this_week,
                        1,
                        target.date,
                    );
                    statistics.completed_values_this_week = TargetHelper::check_prev_week(
                        statistics.completed_values_this_week,
                        target.value,
                        target.date,
                    );
                }
            } else if allow_skip && target.target_type == "skip" {
                statistics.update_streak(&target, days_diff as i32);
                statistics.skipped_count += 1;
                statistics.skipped_count_this_week = TargetHelper::check_prev_week(
                    statistics.skipped_count_this_week,
                    1,
                    target.date,
                );
            } else {
                if statistics.current_streak_start_date.is_some() {
                    statistics.failed_count += (date - prev_date).num_days() as i32 - 1;
                    statistics.current_streak_count = 0;
                    statistics.current_streak_start_date = None;
                }
            }
            prev_date = date;
        }

        return statistics;
    }
}
#[cfg(test)]
mod tests {
    use chrono::Utc;
    use chrono::{Duration, TimeZone};
    use uuid::Uuid;

    use crate::features::habit_target::models::{Target, TargetHelper};

    #[test]
    fn completed_today() {
        let targets = vec![Target {
            id: Uuid::new_v4(),
            habit_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            deleted: false,
            date: Utc::now(),
            created_date: Utc::now(),
            target_type: "done".to_string(),
            value: 100,
        }];

        let result = TargetHelper::calculate_statistics(targets.clone(), false, false, 0);

        assert_eq!(result.completed_today, true, "completed today");
    }

    #[test]
    fn simple_streak() {
        let targets = vec![
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 100,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc.with_ymd_and_hms(2022, 1, 2, 0, 0, 0).unwrap(),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 100,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc.with_ymd_and_hms(2022, 1, 4, 0, 0, 0).unwrap(),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 100,
            },
        ];

        let result = TargetHelper::calculate_statistics(targets.clone(), false, false, 0);

        assert_eq!(
            result.current_streak_start_date,
            Some(Utc.with_ymd_and_hms(2022, 1, 4, 0, 0, 0).unwrap()),
            "current_streak_start_date"
        );

        assert_eq!(result.current_streak_count, 1, "current_streak_count");
        assert_eq!(result.total_count, 3, "total_count");
        assert_eq!(result.completed_count, 3, "completed_count");
        assert_eq!(result.failed_count, 1, "failed_count");
        assert_eq!(result.skipped_count, 0, "skipped_count");
        assert_eq!(result.total_values_count, 300, "total_values_count");
        assert_eq!(result.completed_today, false, "completed_today");
    }

    #[test]
    fn only_skips() {
        let targets = vec![
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "skip".to_string(),
                value: 0,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc.with_ymd_and_hms(2022, 1, 4, 0, 0, 0).unwrap(),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "skip".to_string(),
                value: 0,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc.with_ymd_and_hms(2022, 1, 5, 0, 0, 0).unwrap(),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "skip".to_string(),
                value: 0,
            },
        ];

        let result = TargetHelper::calculate_statistics(targets.clone(), true, false, 0);

        assert_eq!(
            result.current_streak_start_date,
            Some(Utc.with_ymd_and_hms(2022, 1, 4, 0, 0, 0).unwrap()),
            "current_streak_start_date"
        );

        assert_eq!(result.current_streak_count, 2, "current_streak_count");
        assert_eq!(result.total_count, 3, "total_count");
        assert_eq!(result.completed_count, 0, "completed_count");
        assert_eq!(result.failed_count, 2, "failed_count");
        assert_eq!(result.skipped_count, 3, "skipped_count");
        assert_eq!(result.total_values_count, 0, "total_values_count");
    }

    #[test]
    fn skip_targets() {
        let targets = vec![
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 50,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc.with_ymd_and_hms(2022, 1, 3, 0, 0, 0).unwrap(),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 50,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc.with_ymd_and_hms(2022, 1, 4, 0, 0, 0).unwrap(),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "skip".to_string(),
                value: 50,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc.with_ymd_and_hms(2022, 1, 7, 0, 0, 0).unwrap(),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 50,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc.with_ymd_and_hms(2022, 1, 8, 0, 0, 0).unwrap(),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "skip".to_string(),
                value: 0,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc.with_ymd_and_hms(2022, 1, 9, 0, 0, 0).unwrap(),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 50,
            },
        ];

        let result = TargetHelper::calculate_statistics(targets.clone(), true, false, 0);

        assert_eq!(result.current_streak_count, 3, "current_streak_count");
        assert_eq!(result.total_count, 6, "total_count");
        assert_eq!(result.completed_count, 4, "completed_count");
        assert_eq!(result.failed_count, 3, "failed_count");
        assert_eq!(result.skipped_count, 2, "skipped_count");
        assert_eq!(result.total_values_count, 200, "total_values_count");
    }

    #[test]
    fn partial_completion() {
        let targets = vec![
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc.with_ymd_and_hms(2022, 1, 2, 0, 0, 0).unwrap(),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 100,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc.with_ymd_and_hms(2022, 1, 5, 0, 0, 0).unwrap(),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 50,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc.with_ymd_and_hms(2022, 1, 6, 0, 0, 0).unwrap(),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "skip".to_string(),
                value: 0,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc.with_ymd_and_hms(2022, 1, 7, 0, 0, 0).unwrap(),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 50,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc.with_ymd_and_hms(2022, 1, 8, 0, 0, 0).unwrap(),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 100,
            },
        ];

        let result = TargetHelper::calculate_statistics(targets.clone(), true, true, 100);

        assert_eq!(result.current_streak_count, 4, "current_streak_count");
        assert_eq!(result.total_count, 5, "total_count");
        assert_eq!(result.completed_count, 2, "completed_count");
        assert_eq!(result.failed_count, 2, "failed_count");
        assert_eq!(result.skipped_count, 1, "skipped_count");
        assert_eq!(result.total_values_count, 300, "total_values_count");
    }

    #[test]
    fn over_goal_completion() {
        let targets = vec![
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc.with_ymd_and_hms(2022, 1, 2, 0, 0, 0).unwrap(),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 150,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc.with_ymd_and_hms(2022, 1, 5, 0, 0, 0).unwrap(),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 50,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc.with_ymd_and_hms(2022, 1, 6, 0, 0, 0).unwrap(),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "skip".to_string(),
                value: 75,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc.with_ymd_and_hms(2022, 1, 7, 0, 0, 0).unwrap(),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 150,
            },
        ];

        let result = TargetHelper::calculate_statistics(targets.clone(), true, true, 100);

        assert_eq!(result.current_streak_count, 3, "current_streak_count");
        assert_eq!(result.total_count, 4, "total_count");
        assert_eq!(result.completed_count, 2, "completed_count");
        assert_eq!(result.failed_count, 2, "failed_count");
        assert_eq!(result.skipped_count, 1, "skipped_count");
        assert_eq!(result.total_values_count, 350, "total_values_count");
    }

    #[test]
    fn current_streak_calculation() {
        let targets = vec![
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc::now() - Duration::days(8),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 150,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc::now() - Duration::days(7),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 150,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc::now() - Duration::days(6),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 50,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc::now() - Duration::days(5),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 75,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc::now() - Duration::days(4),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 150,
            },
        ];

        let result = TargetHelper::calculate_statistics(targets.clone(), false, false, 100);

        assert_eq!(result.current_streak_count, 5, "current_streak_count");
        assert_eq!(
            result.current_streak_count_this_week, 3,
            "current_streak_count_this_week"
        );
        assert_eq!(result.current_streak_values, 575, "current_streak_values");
        assert_eq!(
            result.current_streak_values_this_week, 275,
            "current_streak_values_this_week"
        );
    }

    #[test]
    fn this_week_calculation() {
        let targets = vec![
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc::now() - Duration::days(11),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 100,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc::now() - Duration::days(10),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "skip".to_string(),
                value: 100,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc::now() - Duration::days(8),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 100,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc::now() - Duration::days(7),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "skip".to_string(),
                value: 100,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc::now() - Duration::days(6),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 100,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc::now() - Duration::days(5),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 100,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc::now() - Duration::days(4),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "skip".to_string(),
                value: 100,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc::now() - Duration::days(2),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 100,
            },
            Target {
                id: Uuid::new_v4(),
                habit_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                deleted: false,
                date: Utc::now() - Duration::days(1),
                created_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: "done".to_string(),
                value: 75,
            },
        ];

        let result = TargetHelper::calculate_statistics(targets.clone(), true, true, 100);

        assert_eq!(result.failed_count, 2, "failed_count");
        assert_eq!(result.failed_count_this_week, 1, "failed_count_this_week");
        assert_eq!(result.skipped_count, 3, "skipped_count");
        assert_eq!(result.skipped_count_this_week, 1, "skipped_count_this_week");
        assert_eq!(result.total_count, 9, "total_count");
        assert_eq!(result.total_count_this_week, 5, "total_count_this_week");
        assert_eq!(result.total_values_count, 575, "total_values_count");
        assert_eq!(
            result.total_values_count_this_week, 375,
            "total_values_count_this_week"
        );
        assert_eq!(result.completed_count, 5, "completed_count");
        assert_eq!(
            result.completed_count_this_week, 3,
            "completed_count_this_week"
        );
        assert_eq!(result.completed_values, 500, "completed_values");
        assert_eq!(
            result.completed_values_this_week, 300,
            "completed_values_this_week"
        );
    }
}
