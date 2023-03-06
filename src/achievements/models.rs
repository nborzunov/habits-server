use crate::habits::models::HabitsAchievement;
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "camelCase", untagged)]
pub enum AchievementKey {
    Habits(HabitsAchievement),
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum AchievementType {
    Habits,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    pub habit_id: String,
    pub habit_title: String,
    pub progress: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct Achievement {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub key: AchievementKey,
    pub achievement_type: AchievementType,
    pub user_id: ObjectId,
    pub origin_ref: ObjectId,
    pub completed_date: Option<DateTime<Utc>>,
    pub completed: bool,
    pub progress: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AchievementResult {
    pub key: AchievementKey,
    pub achievement_type: AchievementType,
    pub completed: bool,
    pub completed_date: Option<DateTime<Utc>>,
    pub progress: Vec<Progress>,
}
