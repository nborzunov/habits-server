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

    pub fn get_streak_and_failures(
        targets: Vec<TargetDetails>,
        allow_skip: bool,
    ) -> (Vec<TargetDetails>, i32) {
        let mut targets = targets.clone();
        targets.reverse();

        let last_done = targets
            .iter()
            .position(|target| matches!(target.target_type, TargetType::Done))
            .unwrap();

        let mut streak_targets = Vec::new();
        streak_targets.push(targets[last_done].clone());
        targets.drain(last_done..last_done + 1);
        let mut prev_date = streak_targets[0].date.date_naive();
        let mut failed_days: i64 = 0;

        for target in targets {
            let date = target.date.date_naive();

            if (prev_date - date).num_days() == 1
                && (matches!(target.clone().target_type, TargetType::Done)
                    || allow_skip && matches!(target.clone().target_type, TargetType::Skip))
            {
                streak_targets.push(target);
            } else {
                failed_days += (prev_date - date).num_days() - 1;
            }
            prev_date = date
        }

        (streak_targets, failed_days as i32)
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

    pub fn get_skipped(targets: &Vec<TargetDetails>) -> i32 {
        let mut skipped = 0;
        for target in targets {
            if matches!(target.target_type, TargetType::Skip) {
                skipped += 1;
            }
        }
        skipped
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
