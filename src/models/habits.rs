use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::targets::{Target, TargetDetails, TargetType};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Habit {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    title: String,
    periodicity: Periodicity,
    periodicity_value: Option<DaysSequence>,
    pub created_date: DateTime<Utc>,
    goal: i32,
    goal_type: GoalType,
    pub allow_skip: bool,
    pub allow_partial_completion: bool,
    pub allow_over_goal_completion: bool,
    pub archived: bool,
}

impl Habit {
    pub fn new(data: &HabitData, user_id: ObjectId) -> Self {
        Habit {
            id: None,
            user_id: user_id.clone(),
            title: data.title.clone(),
            periodicity: data.periodicity.clone(),
            periodicity_value: data.periodicity_value.clone(),
            created_date: Utc::now(),
            goal: data.goal,
            goal_type: data.goal_type.clone(),
            allow_skip: data.allow_skip,
            allow_partial_completion: data.allow_partial_completion,
            allow_over_goal_completion: data.allow_over_goal_completion,
            archived: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HabitDetails {
    pub id: String,
    pub user_id: String,
    title: String,
    periodicity: Periodicity,
    periodicity_value: Option<DaysSequence>,
    created_date: DateTime<Utc>,
    goal: i32,
    goal_type: GoalType,
    allow_skip: bool,
    allow_partial_completion: bool,
    allow_over_goal_completion: bool,
    start_date: Option<DateTime<Utc>>,
    completed_today: bool,
    current_streak: i32,
    current_streak_start_date: Option<DateTime<Utc>>,
    completed_targets: i32,
    failed_targets: i32,
    total_targets: i32,
    skipped_targets: i32,
    archived: bool,

    pub targets: Vec<TargetDetails>,
}

impl HabitDetails {
    pub fn parse(h: &Habit, mut targets: Vec<TargetDetails>) -> HabitDetails {
        targets.sort_by_key(|t| t.date.clone());

        let (current_streak_targets, failed_targets) =
            Target::get_streak_and_failures(targets.clone(), h.allow_skip);

        HabitDetails {
            id: h.id.clone().unwrap().to_string(),
            user_id: h.id.clone().unwrap().to_string(),
            title: h.title.clone(),
            periodicity: h.periodicity.clone(),
            periodicity_value: h.periodicity_value.clone(),
            created_date: h.created_date.clone(),
            start_date: Self::get_start_date(&targets),
            goal: h.goal.clone(),
            goal_type: h.goal_type.clone(),
            allow_skip: h.allow_skip,
            allow_partial_completion: h.allow_partial_completion,
            allow_over_goal_completion: h.allow_over_goal_completion,
            targets: targets.clone(),
            archived: h.archived,
            completed_today: current_streak_targets
                .iter()
                .any(|target| target.date.date_naive() == Utc::now().date_naive()),
            current_streak: current_streak_targets.len() as i32,
            current_streak_start_date: match current_streak_targets
                .iter()
                .find(|t| matches!(t.target_type, TargetType::Done))
            {
                Some(t) => Some(t.date.clone()),
                None => None,
            },
            completed_targets: Target::get_completed(&targets),
            failed_targets,
            total_targets: Target::get_total(&targets),
            skipped_targets: Target::get_skipped(&targets),
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
    periodicity_value: Option<DaysSequence>,
    goal: i32,
    goal_type: GoalType,
    allow_skip: bool,
    allow_partial_completion: bool,
    allow_over_goal_completion: bool,
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
pub struct DaysSequence(pub Vec<DayOfTheWeek>);

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
pub enum GoalType {
    Times,
    Mins,
}
