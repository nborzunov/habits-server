use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::targets::{Target, TargetDetails};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Habit {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    title: String,
    periodicity: Periodicity,
    periodicity_value: Option<CustomPeriodicityValue>,
    activity_type: ActivityType,
    activity_counter_value: Option<ActivityCounterValue>,
    created_date: DateTime<Utc>,
    start_date: Option<DateTime<Utc>>,
    goal: i32,
    goal_type: GoalType,
    pub targets: Vec<ObjectId>,
    // TODO: allow skipping targets
}

impl Habit {
    pub fn new(data: &HabitData) -> Self {
        Habit {
            id: None,
            title: data.title.clone(),
            periodicity: data.periodicity.clone(),
            periodicity_value: data.periodicity_value.clone(),
            activity_type: data.activity_type.clone(),
            activity_counter_value: data.activity_counter_value.clone(),
            created_date: Utc::now(),
            goal: data.goal,
            goal_type: data.goal_type.clone(),
            start_date: None,
            targets: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HabitDetails {
    pub id: String,
    title: String,
    periodicity: Periodicity,
    periodicity_value: Option<CustomPeriodicityValue>,
    activity_type: ActivityType,
    activity_counter_value: Option<ActivityCounterValue>,
    created_date: DateTime<Utc>,

    goal: i32,
    goal_type: GoalType,
    start_date: Option<DateTime<Utc>>,
    completed_today: bool,

    current_streak: i32,
    current_streak_start_date: Option<DateTime<Utc>>,
    completed_targets: i32,
    failed_targets: i32,
    total_targets: i32,

    pub targets: Vec<TargetDetails>,
}

impl HabitDetails {
    // TODO: fix calculation of streak
    pub fn parse(h: &Habit, targets: Vec<TargetDetails>) -> HabitDetails {
        let current_streak_targets = Target::get_streak(&targets);

        let current_streak_start_date = match current_streak_targets.first() {
            Some(target) => Some(target.date),
            None => None,
        };

        let completed_today = current_streak_targets.iter().any(|target| target.date.date_naive() == Utc::now().date_naive());
        let completed_targets = Target::get_completed(&targets);
        let failed_targets = Target::get_failed(&targets);
        let total_targets = Target::get_total(&targets);

        HabitDetails {
            id: h.id.clone().expect("Failed to parse habit id").to_string(),
            title: h.title.clone(),
            periodicity: h.periodicity.clone(),
            periodicity_value: h.periodicity_value.clone(),
            activity_type: h.activity_type.clone(),
            activity_counter_value: h.activity_counter_value.clone(),
            created_date: h.created_date.clone(),
            start_date: Self::get_start_date(&targets),
            goal: h.goal.clone(),
            goal_type: h.goal_type.clone(),
            targets: targets.clone(),
            completed_today,

            current_streak: current_streak_targets.len() as i32,
            current_streak_start_date,
            completed_targets,
            failed_targets,
            total_targets,
        }
    }

    pub fn get_start_date(targets: &Vec<TargetDetails>) -> Option<DateTime<Utc>> {
        if targets.len() == 0 {
            return None;
        }
        return Some(targets[0].date.clone());
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HabitData {
    title: String,
    periodicity: Periodicity,
    periodicity_value: Option<CustomPeriodicityValue>,
    activity_type: ActivityType,
    activity_counter_value: Option<ActivityCounterValue>,
    goal: i32,
    goal_type: GoalType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Periodicity {
    Daily,
    Weekly,
    Monthly,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomPeriodicityValue(Vec<DayOfTheWeek>);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DayOfTheWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActivityType {
    Boolean,
    Counter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityCounterValue(i32);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GoalType {
    Times,
    Mins,
}



