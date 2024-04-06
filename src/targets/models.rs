use chrono::{DateTime, TimeZone, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::cmp::max;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub habit_id: ObjectId,
    pub date: DateTime<Utc>,
    pub create_date: DateTime<Utc>,
    pub target_type: TargetType,
    pub value: i32,
    pub deleted: bool,
}

impl Target {
    pub fn new(data: &TargetData) -> Self {
        Target {
            id: data.id,
            habit_id: data.habit_id.clone(),
            date: data.date.clone(),
            create_date: Utc::now(),
            target_type: data.target_type.clone(),
            value: data.value,
            deleted: false,
        }
    }

    fn check_prev_week(value: i32, delta: i32, date: DateTime<Utc>) -> i32 {
        let today = Utc::now().date_naive();
        let days_diff = (today - date.date_naive()).num_days();

        if days_diff < 7 && days_diff >= 0 {
            return value + delta;
        }
        return value;
    }

    pub fn calculate_statistics(
        targets: Vec<TargetDetails>,
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

            if matches!(target.target_type, TargetType::Done) {
                if date == today {
                    statistics.completed_today = true;
                }

                statistics.update_streak(&target, days_diff as i32);
                statistics.total_values_count += target.value;
                statistics.total_values_count_this_week = Target::check_prev_week(
                    statistics.total_values_count_this_week,
                    target.value,
                    target.date,
                );

                if !allow_partial_completion || target.value >= daily_goal {
                    statistics.completed_count += 1;
                    statistics.completed_values += target.value;
                    statistics.completed_count_this_week = Target::check_prev_week(
                        statistics.completed_count_this_week,
                        1,
                        target.date,
                    );
                    statistics.completed_values_this_week = Target::check_prev_week(
                        statistics.completed_values_this_week,
                        target.value,
                        target.date,
                    );
                }
            } else if allow_skip && matches!(target.target_type, TargetType::Skip) {
                statistics.update_streak(&target, days_diff as i32);
                statistics.skipped_count += 1;
                statistics.skipped_count_this_week =
                    Target::check_prev_week(statistics.skipped_count_this_week, 1, target.date);
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
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

    fn update_streak(self: &mut Self, target: &TargetDetails, days_diff: i32) {
        if self.current_streak_start_date.is_none() {
            self.current_streak_count = 1;
            self.current_streak_values = target.value;
            self.current_streak_start_date = Some(target.date);
            self.current_streak_count_this_week = Target::check_prev_week(0, 1, target.date);
            self.current_streak_values_this_week =
                Target::check_prev_week(0, target.value, target.date);
            self.max_streak_count = max(self.max_streak_count, self.current_streak_count);
        } else if days_diff == 1 {
            self.current_streak_count += 1;
            self.current_streak_values += target.value;
            self.current_streak_count_this_week =
                Target::check_prev_week(self.current_streak_count_this_week, 1, target.date);
            self.current_streak_values_this_week = Target::check_prev_week(
                self.current_streak_values_this_week,
                target.value,
                target.date,
            );
            self.max_streak_count = max(self.max_streak_count, self.current_streak_count);
        } else {
            self.failed_count += days_diff - 1;
            self.failed_count_this_week =
                Target::check_prev_week(self.failed_count_this_week, days_diff - 1, target.date);
            self.max_streak_count = max(self.max_streak_count, self.current_streak_count);
            self.prev_streak_count = self.current_streak_count;
            self.current_streak_count = 1;
            self.current_streak_values = target.value;
            self.current_streak_start_date = Some(target.date);
            self.current_streak_count_this_week = Target::check_prev_week(0, 1, target.date);
            self.current_streak_values_this_week =
                Target::check_prev_week(0, target.value, target.date);
        }
        self.total_count += 1;
        self.total_count_this_week =
            Target::check_prev_week(self.total_count_this_week, 1, target.date);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargetDetails {
    pub id: String,
    pub habit_id: String,
    pub date: DateTime<Utc>,
    pub create_date: DateTime<Utc>,
    pub target_type: TargetType,
    pub value: i32,
}

impl TargetDetails {
    pub fn parse(target: &Target) -> Self {
        TargetDetails {
            id: target.id.clone().unwrap().to_hex(),
            habit_id: target.habit_id.to_hex(),
            date: target.date.clone(),
            create_date: target.create_date.clone(),
            target_type: target.target_type.clone(),
            value: target.value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargetData {
    pub id: Option<ObjectId>,
    pub date: DateTime<Utc>,
    pub target_type: TargetType,
    pub habit_id: ObjectId,
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TargetType {
    Done,
    Skip,
    Empty,
}

#[cfg(test)]
mod tests {
    use crate::targets::models::{Target, TargetDetails, TargetType};
    use chrono::Utc;
    use chrono::{Duration, TimeZone};

    #[test]
    fn completed_today() {
        let targets = vec![TargetDetails {
            id: "3".to_string(),
            habit_id: "habit1".to_string(),
            date: Utc::now(),
            create_date: Utc::now(),
            target_type: TargetType::Done,
            value: 100,
        }];

        let result = Target::calculate_statistics(targets.clone(), false, false, 0);

        assert_eq!(result.completed_today, true, "completed today");
    }

    #[test]
    fn simple_streak() {
        let targets = vec![
            TargetDetails {
                id: "1".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 100,
            },
            TargetDetails {
                id: "2".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc.with_ymd_and_hms(2022, 1, 2, 0, 0, 0).unwrap(),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 100,
            },
            TargetDetails {
                id: "3".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc.with_ymd_and_hms(2022, 1, 4, 0, 0, 0).unwrap(),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 100,
            },
        ];

        let result = Target::calculate_statistics(targets.clone(), false, false, 0);

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
            TargetDetails {
                id: "1".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Skip,
                value: 0,
            },
            TargetDetails {
                id: "2".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc.with_ymd_and_hms(2022, 1, 4, 0, 0, 0).unwrap(),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Skip,
                value: 0,
            },
            TargetDetails {
                id: "3".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc.with_ymd_and_hms(2022, 1, 5, 0, 0, 0).unwrap(),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Skip,
                value: 0,
            },
        ];

        let result = Target::calculate_statistics(targets.clone(), true, false, 0);

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
            TargetDetails {
                id: "6".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 50,
            },
            TargetDetails {
                id: "6".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc.with_ymd_and_hms(2022, 1, 3, 0, 0, 0).unwrap(),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 50,
            },
            TargetDetails {
                id: "1".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc.with_ymd_and_hms(2022, 1, 4, 0, 0, 0).unwrap(),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Skip,
                value: 50,
            },
            TargetDetails {
                id: "1".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc.with_ymd_and_hms(2022, 1, 7, 0, 0, 0).unwrap(),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 50,
            },
            TargetDetails {
                id: "2".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc.with_ymd_and_hms(2022, 1, 8, 0, 0, 0).unwrap(),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Skip,
                value: 0,
            },
            TargetDetails {
                id: "2".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc.with_ymd_and_hms(2022, 1, 9, 0, 0, 0).unwrap(),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 50,
            },
        ];

        let result = Target::calculate_statistics(targets.clone(), true, false, 0);

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
            TargetDetails {
                id: "6".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc.with_ymd_and_hms(2022, 1, 2, 0, 0, 0).unwrap(),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 100,
            },
            TargetDetails {
                id: "1".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc.with_ymd_and_hms(2022, 1, 5, 0, 0, 0).unwrap(),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 50,
            },
            TargetDetails {
                id: "2".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc.with_ymd_and_hms(2022, 1, 6, 0, 0, 0).unwrap(),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Skip,
                value: 0,
            },
            TargetDetails {
                id: "2".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc.with_ymd_and_hms(2022, 1, 7, 0, 0, 0).unwrap(),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 50,
            },
            TargetDetails {
                id: "2".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc.with_ymd_and_hms(2022, 1, 8, 0, 0, 0).unwrap(),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 100,
            },
        ];

        let result = Target::calculate_statistics(targets.clone(), true, true, 100);

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
            TargetDetails {
                id: "6".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc.with_ymd_and_hms(2022, 1, 2, 0, 0, 0).unwrap(),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 150,
            },
            TargetDetails {
                id: "1".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc.with_ymd_and_hms(2022, 1, 5, 0, 0, 0).unwrap(),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 50,
            },
            TargetDetails {
                id: "2".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc.with_ymd_and_hms(2022, 1, 6, 0, 0, 0).unwrap(),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Skip,
                value: 75,
            },
            TargetDetails {
                id: "2".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc.with_ymd_and_hms(2022, 1, 7, 0, 0, 0).unwrap(),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 150,
            },
        ];

        let result = Target::calculate_statistics(targets.clone(), true, true, 100);

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
            TargetDetails {
                id: "6".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc::now() - Duration::days(8),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 150,
            },
            TargetDetails {
                id: "6".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc::now() - Duration::days(7),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 150,
            },
            TargetDetails {
                id: "1".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc::now() - Duration::days(6),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 50,
            },
            TargetDetails {
                id: "2".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc::now() - Duration::days(5),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 75,
            },
            TargetDetails {
                id: "2".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc::now() - Duration::days(4),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 150,
            },
        ];

        let result = Target::calculate_statistics(targets.clone(), false, false, 100);

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
            TargetDetails {
                id: "6".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc::now() - Duration::days(11),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 100,
            },
            TargetDetails {
                id: "6".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc::now() - Duration::days(10),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Skip,
                value: 100,
            },
            TargetDetails {
                id: "6".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc::now() - Duration::days(8),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 100,
            },
            TargetDetails {
                id: "6".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc::now() - Duration::days(7),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Skip,
                value: 100,
            },
            TargetDetails {
                id: "1".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc::now() - Duration::days(6),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 100,
            },
            TargetDetails {
                id: "2".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc::now() - Duration::days(5),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 100,
            },
            TargetDetails {
                id: "2".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc::now() - Duration::days(4),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Skip,
                value: 100,
            },
            TargetDetails {
                id: "2".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc::now() - Duration::days(2),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 100,
            },
            TargetDetails {
                id: "2".to_string(),
                habit_id: "habit1".to_string(),
                date: Utc::now() - Duration::days(1),
                create_date: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
                target_type: TargetType::Done,
                value: 75,
            },
        ];

        let result = Target::calculate_statistics(targets.clone(), true, true, 100);

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
