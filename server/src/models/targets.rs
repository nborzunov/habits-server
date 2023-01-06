use chrono::{DateTime, Utc};
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

    pub fn get_streak(targets: &Vec<TargetDetails>) -> Vec<&TargetDetails> {
        let mut streak = vec![];
        let mut streak_started = false;
        for target in targets {
            if matches!(target.target_type, TargetType::Done) {
                streak_started = true;
                streak.push(target);
            } else if streak_started {
                break;
            }
        }
        streak
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

    pub fn get_failed(targets: &Vec<TargetDetails>) -> i32 {
        let mut failed = 0;
        for target in targets {
            if matches!(target.target_type, TargetType::Skip) {
                failed += 1;
            }
        }
        failed
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

