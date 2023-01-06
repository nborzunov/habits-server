use std::collections::HashMap;

use chrono::{DateTime, Duration, NaiveDate, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::habits::Habit;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub habit_id: ObjectId,
    pub date: DateTime<Utc>,
    pub create_date: DateTime<Utc>,
    pub target_type: TargetType,
}

impl Target {
    pub fn new(data: &TargetData) -> Self {
        Target {
            id: data.id,
            habit_id: data.habit_id.clone(),
            date: data.date.clone(),
            create_date: Utc::now(),
            target_type: data.target_type.clone(),
        }
    }

    pub fn get_streak(habit: &Habit, mut targets: Vec<TargetDetails>, first: Option<TargetDetails>) -> (Vec<TargetDetails>, i32) {
        let mut streak = vec![];
        let mut failed = 0;
        targets.reverse();

        if targets.len() == 0 {
            return (streak, failed);
        }
        let mut last = targets[0].clone().date.date_naive();
        let first = first.unwrap().date.date_naive();
        let mut skips: Vec<&TargetDetails> = vec![];
        let mut started = false;
        let mut targets_map: HashMap<NaiveDate, TargetDetails> = HashMap::new();
        for target in targets.iter() {
            targets_map.insert(target.date.clone().date_naive(), target.clone());
        }

        // TODO: handle if there are series start with skip
        while last != first {
            match targets_map.get(&last) {
                Some(target) => {
                    if target.date.date_naive() == last {
                        if habit.allow_skip && matches!(target.clone().target_type, TargetType::Skip) {
                            skips.push(target);
                        } else if matches!(target.clone().target_type, TargetType::Done) {
                            started = true;
                            for &skip in skips.iter() {
                                streak.push(skip.clone());
                            }
                            skips.clear();
                            streak.push(target.clone());
                        }
                    }
                }
                None => {
                    if started {
                        failed += 1;
                    }
                }
            }
            last -= Duration::days(1);
        };


        streak.reverse();
        return (streak, failed);
    }

    pub fn get_completed(targets: &Vec<TargetDetails>) -> i32 {
        let mut completed = 0;
        for target in targets {
            if matches!(target.target_type, TargetType::Done) {
                completed += 1;
            }
        }
        completed
    }

    pub fn get_total(targets: &Vec<TargetDetails>) -> i32 {
        let mut total = 0;
        for target in targets {
            if !matches!(target.target_type, TargetType::Empty) {
                total += 1;
            }
        }
        total
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
}

impl TargetDetails {
    pub fn parse(target: &Target) -> Self {
        TargetDetails {
            id: target.id.clone().unwrap().to_hex(),
            habit_id: target.habit_id.to_hex(),
            date: target.date.clone(),
            create_date: target.create_date.clone(),
            target_type: target.target_type.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TargetData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub date: DateTime<Utc>,
    pub target_type: TargetType,
    pub habit_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TargetType {
    Done,
    Skip,
    Empty,
}

