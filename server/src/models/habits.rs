use std::sync::Mutex;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// use crate::schema::{habits};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TargetType {
    Done,
    Skip,
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    date: DateTime<Utc>,
    create_date: DateTime<Utc>,
    target_type: TargetType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Habit {
    id: String,
    title: String,
    periodicity: Periodicity,
    periodicity_value: Option<CustomPeriodicityValue>,
    activity_type: ActivityType,
    activity_counter_value: Option<ActivityCounterValue>,
    created_date: DateTime<Utc>,
    goal: i32,
    goal_type: GoalType,
    start_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabitDetails {
    id: String,
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

    targets: Vec<Target>,
}

impl Habit {
    pub fn new(data: &HabitModel) -> Self {
        Habit {
            id: Uuid::new_v4().to_string(),
            title: data.title.clone(),
            periodicity: data.periodicity.clone(),
            periodicity_value: data.periodicity_value.clone(),
            activity_type: data.activity_type.clone(),
            activity_counter_value: data.activity_counter_value.clone(),
            created_date: Utc::now(),
            goal: data.goal,
            goal_type: data.goal_type.clone(),
            start_date: None,
            // completed_today: false,
            // current_streak: 0,
            // current_streak_start_date: None,
            // completed_targets: 0,
            // failed_targets: 0,
            // total_targets: 0,
            // targets: vec![]
        }
    }
}

pub type HabitsValue = Mutex<Vec<Habit>>;

#[derive(Serialize, Deserialize)]
pub struct Habits(HabitsValue);

impl Habits {
    pub fn new() -> Self {
        Habits(Mutex::new(Vec::new()))
    }
    pub fn inner(&self) -> &HabitsValue {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut HabitsValue {
        &mut self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HabitModel {
    title: String,
    periodicity: Periodicity,
    periodicity_value: Option<CustomPeriodicityValue>,
    activity_type: ActivityType,
    activity_counter_value: Option<ActivityCounterValue>,
    goal: i32,
    goal_type: GoalType,
}

