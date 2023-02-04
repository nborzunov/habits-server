use chrono::{DateTime, TimeZone, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

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

    pub fn calculate_statistics(
        targets: Vec<TargetDetails>,
        allow_skip: bool,
        allow_partial_completion: bool,
        daily_goal: i32,
    ) -> TargetStatistics {
        let today = Utc::now().date_naive();
        let mut current_streak_start_date: Option<DateTime<Utc>> = None;
        let mut current_streak_count = 0;
        let mut failed_count = 0;
        let mut skipped_count = 0;
        let mut total_count = 0;
        let mut total_values_count = 0;
        let mut completed_count = 0;
        let mut completed_today = false;

        if targets.len() == 0 {
            return TargetStatistics {
                current_streak_start_date,
                current_streak_count,
                failed_count,
                skipped_count,
                total_count,
                total_values_count,
                completed_count,
                completed_today,
            };
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
                    completed_today = true;
                }

                if current_streak_start_date.is_none() {
                    current_streak_count = 1;
                    current_streak_start_date = Some(target.date);
                } else if days_diff == 1 {
                    current_streak_count += 1;
                } else {
                    failed_count += days_diff as i32 - 1;
                    current_streak_count = 1;
                    current_streak_start_date = Some(target.date);
                }
                total_count += 1;
                total_values_count += target.value;

                if !allow_partial_completion || target.value >= daily_goal {
                    completed_count += 1;
                }
            } else if allow_skip && matches!(target.target_type, TargetType::Skip) {
                if current_streak_start_date.is_none() {
                    current_streak_count = 1;
                    current_streak_start_date = Some(target.date);
                } else if days_diff == 1 {
                    current_streak_count += 1;
                } else {
                    failed_count += days_diff as i32 - 1;
                    current_streak_count = 1;
                    current_streak_start_date = Some(target.date);
                }
                total_count += 1;
                skipped_count += 1;
            } else {
                if current_streak_start_date.is_some() {
                    failed_count += (date - prev_date).num_days() as i32 - 1;
                    current_streak_count = 0;
                    current_streak_start_date = None;
                }
            }
            prev_date = date;
        }

        TargetStatistics {
            current_streak_start_date,
            current_streak_count,
            failed_count,
            skipped_count,
            total_count,
            total_values_count,
            completed_count,
            completed_today,
        }
    }
}

pub struct TargetStatistics {
    pub current_streak_start_date: Option<DateTime<Utc>>,
    pub current_streak_count: i32,
    pub failed_count: i32,
    pub skipped_count: i32,
    pub total_count: i32,
    pub total_values_count: i32,
    pub completed_count: i32,
    pub completed_today: bool,
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
#[serde(rename_all = "lowercase")]
pub enum TargetType {
    Done,
    Skip,
    Empty,
}

#[cfg(test)]
mod tests {
    use crate::models::targets::{Target, TargetDetails, TargetStatistics, TargetType};
    use chrono::TimeZone;
    use chrono::Utc;

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
}
