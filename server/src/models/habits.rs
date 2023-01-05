use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Habit {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
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
    targets: Vec<Target>,
}

impl Habit {
    pub fn get_details(&self) -> HabitDetails {
        let current_streak_targets = Target::get_streak(&self.targets);

        let current_streak_start_date = match current_streak_targets.first() {
            Some(target) => Some(target.date),
            None => None,
        };

        let completed_today = current_streak_targets.iter().any(|target| target.date.date_naive() == Utc::now().date_naive());
        let completed_targets = Target::get_completed(&self.targets);
        let failed_targets = Target::get_failed(&self.targets);
        let total_targets = Target::get_total(&self.targets);

        HabitDetails {
            id: self.id.clone().expect("Failed to parse habit id").to_string(),
            title: self.title.clone(),
            periodicity: self.periodicity.clone(),
            periodicity_value: self.periodicity_value.clone(),
            activity_type: self.activity_type.clone(),
            activity_counter_value: self.activity_counter_value.clone(),
            created_date: self.created_date.clone(),
            start_date: self.start_date.clone(),
            goal: self.goal.clone(),
            goal_type: self.goal_type.clone(),
            targets: self.targets.clone(),
            completed_today,

            current_streak: current_streak_targets.len() as i32,
            current_streak_start_date,
            completed_targets,
            failed_targets,
            total_targets,

        }
    }
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TargetType {
    Done,
    Skip,
    Empty,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Target {
    id: ObjectId,
    date: DateTime<Utc>,
    create_date: DateTime<Utc>,
    target_type: TargetType,
}

impl Target {
    pub fn get_streak(targets: &Vec<Target>) -> Vec<&Target> {
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

    pub fn get_completed(targets: &Vec<Target>) -> i32 {
        let mut completed = 0;
        for target in targets {
            if matches!(target.target_type, TargetType::Done) {
                completed += 1;
            }
        }
        completed
    }

    pub fn get_failed(targets: &Vec<Target>) -> i32 {
        let mut failed = 0;
        for target in targets {
            if matches!(target.target_type, TargetType::Skip) {
                failed += 1;
            }
        }
        failed
    }

    pub fn get_total(targets: &Vec<Target>) -> i32 {
        let mut total = 0;
        for target in targets {
            if !matches!(target.target_type, TargetType::Empty) {
                total += 1;
            }
        }
        total
    }
}