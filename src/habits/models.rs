use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::targets::models::{Target, TargetDetails, TargetStatistics, TargetType};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Habit {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub title: String,
    periodicity: Periodicity,
    periodicity_value: Option<DaysSequence>,
    pub created_date: DateTime<Utc>,
    goal: i32,
    goal_type: GoalType,
    pub allow_skip: bool,
    pub allow_partial_completion: bool,
    pub allow_over_goal_completion: bool,
    can_be_finished: bool,
    total_goal: i32,
    pub archived: bool,
    pub deleted: bool,
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
            can_be_finished: data.can_be_finished,
            total_goal: data.total_goal,
            archived: false,
            deleted: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HabitDetails {
    pub id: String,
    pub user_id: String,
    pub title: String,
    periodicity: Periodicity,
    periodicity_value: Option<DaysSequence>,
    created_date: DateTime<Utc>,
    start_date: Option<DateTime<Utc>>,
    goal: i32,
    goal_type: GoalType,
    allow_skip: bool,
    allow_partial_completion: bool,
    allow_over_goal_completion: bool,
    can_be_finished: bool,
    total_goal: i32,
    pub statistics: TargetStatistics,
    archived: bool,
    pub targets: Vec<TargetDetails>,
}

impl HabitDetails {
    pub fn parse(h: &Habit, mut targets: Vec<TargetDetails>) -> HabitDetails {
        targets.sort_by_key(|t| t.date.clone());

        if !h.allow_skip {
            targets.retain(|t| !matches!(t.target_type, TargetType::Skip));
        }

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
            can_be_finished: h.can_be_finished,
            total_goal: h.total_goal,
            targets: targets.clone(),
            archived: h.archived,
            statistics: Target::calculate_statistics(
                targets.clone(),
                h.allow_skip,
                h.allow_partial_completion,
                h.goal,
            ),
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
    can_be_finished: bool,
    total_goal: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Periodicity {
    Daily,
    Weekly,
    Monthly,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaysSequence(pub Vec<DayOfTheWeek>);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub enum GoalType {
    Times,
    Mins,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
#[serde(rename_all = "camelCase")]
pub enum HabitsAchievement {
    StreakStarter,
    HabitFormed,
    ConsistencyChampion,
    HabitualHero,
    HabitMaster,
    HabitProdigy,
    HabitLegend,
    SteadyEddie,
    Relentless,
    Unstoppable,
    SurpassingLimits,
    Perseverance,
    ComebackKid,
}

impl HabitsAchievement {
    pub fn goal(&self) -> Option<i32> {
        match self {
            Self::StreakStarter => Some(3),
            Self::HabitFormed => Some(7),
            Self::ConsistencyChampion => Some(14),
            Self::HabitualHero => Some(30),
            Self::HabitMaster => Some(60),
            Self::HabitProdigy => Some(90),
            Self::HabitLegend => Some(180),
            Self::SteadyEddie => Some(21),
            Self::Relentless => Some(30),
            Self::Unstoppable => Some(60),
            _ => None,
        }
    }

    pub fn get_all() -> Vec<Self> {
        vec![
            Self::StreakStarter,
            Self::HabitFormed,
            Self::ConsistencyChampion,
            Self::HabitualHero,
            Self::HabitMaster,
            Self::HabitProdigy,
            Self::HabitLegend,
            Self::SteadyEddie,
            Self::Relentless,
            Self::Unstoppable,
            Self::SurpassingLimits,
            Self::Perseverance,
            Self::ComebackKid,
        ]
    }

    pub fn check(achievement_key: &HabitsAchievement, habit: HabitDetails) -> (bool, i32) {
        let mut completed = false;

        return match achievement_key {
            HabitsAchievement::StreakStarter
            | HabitsAchievement::HabitFormed
            | HabitsAchievement::ConsistencyChampion
            | HabitsAchievement::HabitualHero
            | HabitsAchievement::HabitMaster
            | HabitsAchievement::HabitProdigy
            | HabitsAchievement::HabitLegend => {
                let goal = achievement_key.goal();
                if goal.is_some() && habit.statistics.max_streak_count >= goal.unwrap() {
                    completed = true;
                }
                (completed, habit.statistics.max_streak_count)
            }
            HabitsAchievement::SteadyEddie
            | HabitsAchievement::Relentless
            | HabitsAchievement::Unstoppable => {
                let goal = achievement_key.goal();
                if goal.is_some()
                    && habit.statistics.max_streak_count >= goal.unwrap()
                    && habit.statistics.failed_count == 0
                {
                    completed = true;
                }

                (completed, habit.statistics.completed_count)
            }
            HabitsAchievement::SurpassingLimits => {
                if habit.statistics.current_streak_count >= habit.statistics.prev_streak_count
                    && habit.statistics.prev_streak_count > 0
                {
                    completed = true;
                }

                (completed, habit.statistics.current_streak_count)
            }
            HabitsAchievement::Perseverance | HabitsAchievement::ComebackKid => {
                let goal = achievement_key.goal();
                if goal.is_some()
                    && habit.statistics.current_streak_count >= goal.unwrap()
                    && habit.statistics.failed_count > 0
                {
                    completed = true;
                }

                (completed, habit.statistics.current_streak_count)
            }
        };
    }
}
